//! Core lightning configuration manager written in rust.
use std::{collections::HashMap, fmt, rc::Rc};

use file::{File, SyncFile};

mod file;
mod parser;

pub type ParsingError = String;

pub trait SyncCLNConf {
    fn parse(&mut self) -> Result<(), ParsingError>;
}

/// core lightning configuration manager
/// that help to parser and create a core
/// lightning configuration with rust.
#[derive(Debug)]
pub struct CLNConf {
    /// collection of field included
    /// inside the conf file.
    ///
    /// `plugin=path/to/bin` is parser as
    /// `key=value`.
    pub filed: HashMap<String, String>,
    /// other conf file included.
    pub includes: Vec<Rc<CLNConf>>,
    path: String,
}

impl CLNConf {
    /// create a new instance of the configuration
    /// file manager.
    pub fn new(path: String) -> Self {
        CLNConf {
            filed: HashMap::new(),
            includes: Vec::new(),
            path,
        }
    }

    /// build a new instance of the parser.
    pub fn parser(&self) -> parser::Parser {
        parser::Parser::new(&self.path)
    }

    pub fn add_conf(&mut self, key: &str, val: &str) {
        self.filed.insert(key.to_owned(), val.to_owned());
    }

    pub fn add_subconf(&mut self, conf: CLNConf) {
        self.includes.push(conf.into());
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        let content = format!("{self}");
        let file = File::new(&self.path);
        file.write(&content)?;
        Ok(())
    }
}

impl SyncCLNConf for CLNConf {
    fn parse(&mut self) -> Result<(), ParsingError> {
        let parser = self.parser();
        parser.parse(self)?;
        Ok(())
    }
}

impl fmt::Display for CLNConf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        for field in self.filed.keys() {
            let value = self.filed.get(field).unwrap();
            content += format!("{field}={value}\n").as_str();
        }

        for include in &self.includes {
            content += format!("{include}\n").as_str();
        }

        writeln!(f, "{content}")
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{remove_file, File};
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{CLNConf, SyncCLNConf};

    fn get_conf_path() -> String {
        let binding = env::temp_dir();
        let dir = binding.as_os_str().to_str().unwrap();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        format!("{dir}/conf-{}", nanos)
    }

    fn build_file(content: &str) -> Result<String, std::io::Error> {
        let conf = get_conf_path();
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
        let mut conf = CLNConf::new(&path);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.filed.keys().len(), 2);

        assert!(conf.filed.contains_key("plugin"));
        assert!(conf.filed.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_one() {
        let path = get_conf_path();
        let mut conf = CLNConf::new(&path);
        conf.add_conf("plugin", "/some/path");
        conf.add_conf("network", "bitcoin");
        let result = conf.flush();
        assert!(result.is_ok());

        let mut conf = CLNConf::new(&path);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.filed.keys().len(), 2);
        println!("{:?}", conf);
        assert!(conf.filed.contains_key("plugin"));
        assert!(conf.filed.contains_key("network"));

        cleanup_file(path.as_str());
    }
}
