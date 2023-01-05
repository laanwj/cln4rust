//! Parser implementation to parse the simple
//! conf syntax for core lightning conf file
use crate::{
    file::{File, SyncFile},
    CLNConf, ParsingError, SyncCLNConf,
};
use albert_stream::{BasicStream, Stream};

pub struct Parser {
    file: File,
}

type Word = String;

impl Parser {
    pub(crate) fn new(file_path: &str) -> Self {
        Parser {
            file: File::new(file_path),
        }
    }

    fn read_and_split(&self) -> Result<Vec<Word>, ParsingError> {
        let content = self.file.read().unwrap();
        let words: Vec<Word> = content
            .split('=')
            .map(|it| it.to_string())
            .collect::<Vec<String>>();
        Ok(words)
    }

    pub fn parse(&self, conf: &mut CLNConf) -> Result<(), ParsingError> {
        let words = self.read_and_split()?;
        let mut stream = BasicStream::<Word>::new(&words);
        self.parse_stream(&mut stream, conf)
    }

    fn parse_stream(
        &self,
        stream: &mut BasicStream<Word>,
        conf: &mut CLNConf,
    ) -> Result<(), ParsingError> {
        while !stream.is_end() {
            self.parse_key_value(stream, conf)?;
        }
        Ok(())
    }

    fn parse_key_value(
        &self,
        stream: &mut BasicStream<Word>,
        conf: &mut CLNConf,
    ) -> Result<(), ParsingError> {
        let key = stream.advance().to_owned();
        let value = stream.advance();
        if key == "include" {
            let mut subconf = CLNConf::new(&value);
            if let Err(err) = subconf.parse() {
                return Err(err);
            }
            conf.add_subconf(subconf);
        } else {
            conf.add_conf(&key, value);
        }
        Ok(())
    }
}
