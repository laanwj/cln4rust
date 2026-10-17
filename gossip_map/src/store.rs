//! Incremental reader for the `gossip_store` file written by core lightning.
//!
//! The file is a version byte followed by records of:
//!
//! ```text
//! u16 flags | u16 len | u32 crc | u32 timestamp | len bytes of message
//! ```
//!
//! While `gossipd` runs it only appends to the file. When it starts, it
//! compacts the store into a new file and appends a `gossip_store_ended`
//! record to the old one. The reader keeps the file open, remembers where
//! it stopped, and reopens the path when it finds the end marker.
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::flags::{
    GOSSIP_STORE_COMPLETED_MINOR_VERSION, GOSSIP_STORE_LEN_COMPLETED_BIT,
    GOSSIP_STORE_LEN_DELETED_BIT, GOSSIP_STORE_LEN_DYING_BIT, GOSSIP_STORE_MAJOR_VERSION,
    GOSSIP_STORE_MAJOR_VERSION_MASK, GOSSIP_STORE_MINOR_VERSION_MASK, WIRE_GOSSIP_STORE_ENDED,
};

const HEADER_LEN: usize = 12;

/// A record of the gossip store.
#[derive(Debug, Clone)]
pub struct GossipStoreRecord {
    /// Offset of the record header in the file.
    pub offset: u64,
    pub flags: u16,
    pub timestamp: u32,
    /// The raw message, starting with its 2 bytes type.
    pub msg: Vec<u8>,
}

impl GossipStoreRecord {
    pub fn msg_type(&self) -> u16 {
        u16::from_be_bytes([self.msg[0], self.msg[1]])
    }

    pub fn is_deleted(&self) -> bool {
        self.flags & GOSSIP_STORE_LEN_DELETED_BIT != 0
    }

    pub fn is_dying(&self) -> bool {
        self.flags & GOSSIP_STORE_LEN_DYING_BIT != 0
    }
}

/// Read the gossip store one record at the time, and pick up
/// the records appended after the last read.
#[derive(Debug)]
pub struct GossipStoreReader {
    path: PathBuf,
    stream: Option<BufReader<File>>,
    version: u8,
    offset: u64,
}

impl GossipStoreReader {
    /// Create a reader for the gossip store at `path`, the file
    /// is opened at the first read.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        GossipStoreReader {
            path: path.as_ref().to_path_buf(),
            stream: None,
            version: 0,
            offset: 0,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Version byte of the open gossip store.
    pub fn version(&self) -> Option<u8> {
        self.stream.as_ref().map(|_| self.version)
    }

    /// Offset of the next record to read.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Close the file, so the next read starts from the beginning.
    pub fn reset(&mut self) {
        self.stream = None;
        self.offset = 0;
    }

    /// Read the next record.
    ///
    /// Returns `None` when all the records written so far have been
    /// read, call it again later to read the records appended by
    /// `gossipd` in the meantime.
    pub fn next_record(&mut self) -> anyhow::Result<Option<GossipStoreRecord>> {
        loop {
            if self.stream.is_none() && !self.open()? {
                return Ok(None);
            }
            let Some(record) = self.read_record()? else {
                return Ok(None);
            };
            if record.msg_type() == WIRE_GOSSIP_STORE_ENDED {
                log::debug!(
                    "gossip store `{}` was rewritten, reopening it",
                    self.path.display()
                );
                self.reset();
                continue;
            }
            return Ok(Some(record));
        }
    }

    fn open(&mut self) -> anyhow::Result<bool> {
        let mut stream = BufReader::new(File::open(&self.path)?);
        let mut version = [0u8; 1];
        if !read_full(&mut stream, &mut version)? {
            // gossipd did not write the version yet.
            return Ok(false);
        }
        let version = version[0];
        if (version as u16 & GOSSIP_STORE_MAJOR_VERSION_MASK) != GOSSIP_STORE_MAJOR_VERSION {
            anyhow::bail!("Invalid gossip store version {version}");
        }
        self.version = version;
        self.offset = 1;
        self.stream = Some(stream);
        Ok(true)
    }

    fn read_record(&mut self) -> anyhow::Result<Option<GossipStoreRecord>> {
        // SAFETY: the stream is opened before reading a record.
        let stream = self.stream.as_mut().unwrap();
        let mut header = [0u8; HEADER_LEN];
        if !read_full(stream, &mut header)? {
            return self.rewind();
        }
        let flags = u16::from_be_bytes([header[0], header[1]]);
        let len = u16::from_be_bytes([header[2], header[3]]) as usize;
        let timestamp = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
        let minor_version = self.version & GOSSIP_STORE_MINOR_VERSION_MASK;
        if minor_version >= GOSSIP_STORE_COMPLETED_MINOR_VERSION
            && flags & GOSSIP_STORE_LEN_COMPLETED_BIT == 0
        {
            // gossipd is still writing the record.
            return self.rewind();
        }
        let mut msg = vec![0u8; len];
        if !read_full(stream, &mut msg)? {
            return self.rewind();
        }
        if msg.len() < 2 {
            anyhow::bail!(
                "Invalid gossip store record of {len} bytes at offset {}",
                self.offset
            );
        }
        let record = GossipStoreRecord {
            offset: self.offset,
            flags,
            timestamp,
            msg,
        };
        self.offset += (HEADER_LEN + len) as u64;
        Ok(Some(record))
    }

    /// Go back to the beginning of the record that is not
    /// fully written yet, so the next read tries it again.
    fn rewind(&mut self) -> anyhow::Result<Option<GossipStoreRecord>> {
        // SAFETY: the stream is opened before reading a record.
        let stream = self.stream.as_mut().unwrap();
        stream.seek(SeekFrom::Start(self.offset))?;
        Ok(None)
    }
}

/// Fill `buff`, returns false if the end of the file comes first.
fn read_full<R: Read>(reader: &mut R, buff: &mut [u8]) -> std::io::Result<bool> {
    let mut filled = 0;
    while filled < buff.len() {
        match reader.read(&mut buff[filled..]) {
            Ok(0) => return Ok(false),
            Ok(n) => filled += n,
            Err(err) if err.kind() == ErrorKind::Interrupted => {}
            Err(err) => return Err(err),
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::io::Write;

    use super::*;
    use crate::flags::{WIRE_GOSSIP_STORE_CHANNEL_AMOUNT, WIRE_GOSSIP_STORE_UUID};

    fn record(flags: u16, msg: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&flags.to_be_bytes());
        out.extend_from_slice(&(msg.len() as u16).to_be_bytes());
        out.extend_from_slice(&0u32.to_be_bytes());
        out.extend_from_slice(&7u32.to_be_bytes());
        out.extend_from_slice(msg);
        out
    }

    fn msg(ty: u16, body: &[u8]) -> Vec<u8> {
        let mut out = ty.to_be_bytes().to_vec();
        out.extend_from_slice(body);
        out
    }

    fn store_path(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("gossip-map-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("gossip_store")
    }

    fn append(path: &Path, buff: &[u8]) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        file.write_all(buff).unwrap();
    }

    fn read_all(reader: &mut GossipStoreReader) -> Vec<GossipStoreRecord> {
        let mut records = vec![];
        while let Some(record) = reader.next_record().unwrap() {
            records.push(record);
        }
        records
    }

    #[test]
    fn read_gossip_store_from_file() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../contrib/gossip_store");
        let mut reader = GossipStoreReader::new(path);
        let mut types: HashMap<u16, usize> = HashMap::new();
        for record in read_all(&mut reader) {
            *types.entry(record.msg_type()).or_default() += 1;
        }
        assert_eq!(reader.version(), Some(14));
        assert_eq!(reader.offset(), std::fs::metadata(path).unwrap().len());
        for ty in [256, 257, 258, WIRE_GOSSIP_STORE_CHANNEL_AMOUNT] {
            assert!(types.contains_key(&ty), "no message with type {ty}");
        }
        assert_eq!(types[&256], types[&WIRE_GOSSIP_STORE_CHANNEL_AMOUNT]);
    }

    #[test]
    fn waits_for_the_completed_bit() {
        let path = store_path("completed");
        let _ = std::fs::remove_file(&path);
        append(&path, &[16]);
        append(
            &path,
            &record(
                GOSSIP_STORE_LEN_COMPLETED_BIT,
                &msg(WIRE_GOSSIP_STORE_UUID, &[1; 32]),
            ),
        );
        let pending_offset = std::fs::metadata(&path).unwrap().len();
        append(&path, &record(0, &msg(258, &[2; 4])));

        let mut reader = GossipStoreReader::new(&path);
        let records = read_all(&mut reader);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].msg_type(), WIRE_GOSSIP_STORE_UUID);
        assert_eq!(records[0].timestamp, 7);
        assert_eq!(reader.offset(), pending_offset);

        // gossipd sets the completed bit with a single byte write.
        let mut file = OpenOptions::new().write(true).open(&path).unwrap();
        file.seek(SeekFrom::Start(pending_offset)).unwrap();
        file.write_all(&[(GOSSIP_STORE_LEN_COMPLETED_BIT >> 8) as u8])
            .unwrap();
        let records = read_all(&mut reader);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].offset, pending_offset);
        assert_eq!(records[0].msg, msg(258, &[2; 4]));

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn resumes_after_a_partial_record() {
        let path = store_path("partial");
        let _ = std::fs::remove_file(&path);
        append(&path, &[14]);
        append(
            &path,
            &record(GOSSIP_STORE_LEN_DELETED_BIT, &msg(258, &[1; 4])),
        );
        let full = record(GOSSIP_STORE_LEN_DYING_BIT, &msg(256, &[2; 8]));
        append(&path, &full[..HEADER_LEN + 3]);

        let mut reader = GossipStoreReader::new(&path);
        let records = read_all(&mut reader);
        assert_eq!(records.len(), 1);
        assert!(records[0].is_deleted());

        append(&path, &full[HEADER_LEN + 3..]);
        let records = read_all(&mut reader);
        assert_eq!(records.len(), 1);
        assert!(records[0].is_dying());
        assert_eq!(records[0].msg, msg(256, &[2; 8]));
        assert!(read_all(&mut reader).is_empty());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn reopens_the_store_after_the_end_marker() {
        let path = store_path("ended");
        let _ = std::fs::remove_file(&path);
        append(&path, &[14]);
        append(&path, &record(0, &msg(257, &[1; 4])));

        let mut reader = GossipStoreReader::new(&path);
        assert_eq!(read_all(&mut reader).len(), 1);

        // gossipd writes the compacted store to a new file,
        // and then marks the old one as ended.
        let mut old = OpenOptions::new().append(true).open(&path).unwrap();
        let new_path = path.with_extension("tmp");
        append(&new_path, &[14]);
        append(&new_path, &record(0, &msg(257, &[1; 4])));
        append(&new_path, &record(0, &msg(258, &[2; 4])));
        std::fs::rename(&new_path, &path).unwrap();
        old.write_all(&record(0, &msg(WIRE_GOSSIP_STORE_ENDED, &[0; 8])))
            .unwrap();

        let records = read_all(&mut reader);
        let types: Vec<u16> = records.iter().map(|record| record.msg_type()).collect();
        assert_eq!(types, vec![257, 258]);
        assert_eq!(reader.offset(), std::fs::metadata(&path).unwrap().len());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn rejects_unknown_major_version() {
        let path = store_path("version");
        let _ = std::fs::remove_file(&path);
        append(&path, &[1 << 5]);
        let mut reader = GossipStoreReader::new(&path);
        assert!(reader.next_record().is_err());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
}
