//! file implementation to read and write content
use std::fs;
use std::io::{Error, Write};

pub trait SyncFile {
    /// sync version to write into the file
    fn write(&self, buff: &str) -> Result<(), Error> {
        let mut writer = fs::File::create(self.path())?;
        write!(writer, "{buff}")?;
        Ok(())
    }

    /// sync version to read the content of the file
    fn read(&self) -> Result<String, Error> {
        let content = fs::read_to_string(self.path())?;
        Ok(content)
    }

    fn path(&self) -> String;
}

pub struct File {
    path: String,
}

impl File {
    pub fn new(path: &str) -> Self {
        File {
            path: path.to_owned(),
        }
    }
}

impl SyncFile for File {
    fn path(&self) -> String {
        self.path.to_owned()
    }
}
