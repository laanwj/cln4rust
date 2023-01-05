//! Core lightning configuration manager written in rust.
use std::{collections::HashMap, rc::Rc};

mod file;
mod parser;

pub type ParsingError = String;

pub trait SyncCLNConf {
    fn parse(&mut self, path: &str) -> Result<(), ParsingError>;
}

/// core lightning configuration manager
/// that help to parser and create a core
/// lightning configuration with rust.
pub struct CLNConf {
    /// collection of field included
    /// inside the conf file.
    ///
    /// `plugin=path/to/bin` is parser as
    /// `key=value`.
    pub filed: HashMap<String, String>,
    /// other conf file included.
    pub includes: Vec<Rc<CLNConf>>,
}

impl CLNConf {
    /// create a new instance of the configuration
    /// file manager.
    pub fn new() -> Self {
        CLNConf {
            filed: HashMap::new(),
            includes: Vec::new(),
        }
    }

    /// build a new instance of the parser.
    pub fn parser(&self, path: &str) -> parser::Parser {
        parser::Parser::new(path)
    }

    pub fn add_conf(&mut self, key: &str, val: &str) {
        self.filed.insert(key.to_owned(), val.to_owned());
    }

    pub fn add_subconf(&mut self, conf: CLNConf) {
        self.includes.push(conf.into());
    }
}

impl SyncCLNConf for CLNConf {
    fn parse(&mut self, path: &str) -> Result<(), ParsingError> {
        let parser = self.parser(path);
        if let Err(err) = parser.parse(self) {
            return Err(err.to_string());
        }
        Ok(())
    }
}

impl Default for CLNConf {
    fn default() -> Self {
        CLNConf::new()
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{remove_file, File};
    use std::io::Write;

    use crate::{CLNConf, SyncCLNConf};

    fn build_file(content: &str) -> Result<String, std::io::Error> {
        let binding = env::temp_dir();
        let dir = binding.as_os_str().to_str().unwrap();
        let conf = format!("{dir}/conf");
        let mut file = File::create(conf.clone())?;
        write!(file, "{content}")?;
        Ok(conf)
    }

    fn cleanup_file(path: &str) {
        remove_file(path).unwrap();
    }

    #[test]
    fn parsing_key_value_one() {
        let path = build_file("plugin=foo\nnetwork=bitcoin");
        assert!(path.is_ok());
        let path = path.unwrap();
        let mut conf = CLNConf::new();
        let result = conf.parse(path.as_str());
        assert!(result.is_ok());
        assert_eq!(conf.filed.keys().len(), 2);

        cleanup_file(path.as_str());
    }
}
