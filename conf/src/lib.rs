//! Core lightning configuration manager written in rust.
use indexmap::IndexMap;
use std::sync::Arc;
use std::{fmt, io};

mod file;
mod parser;

use file::{File, SyncFile};

#[derive(Debug)]
pub struct ParsingError {
    pub core: u64,
    pub cause: String,
}

impl From<io::Error> for ParsingError {
    fn from(value: io::Error) -> Self {
        ParsingError {
            core: 1,
            cause: format!("{value}"),
        }
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

pub trait SyncCLNConf {
    fn parse(&mut self) -> Result<(), ParsingError>;
}

/// core lightning configuration manager
/// that help to parser and create a core
/// lightning configuration with rust.
#[derive(Debug, Clone)]
pub struct CLNConf {
    /// collection of field included
    /// inside the conf file.
    ///
    /// `plugin=path/to/bin` is parser as
    /// `key=value`.
    pub fields: IndexMap<String, Vec<String>>,
    /// other conf file included.
    pub includes: Vec<Arc<CLNConf>>,
    pub path: String,
    create_if_missing: bool,
}

impl CLNConf {
    /// create a new instance of the configuration
    /// file manager.
    pub fn new(path: String, create_if_missing: bool) -> Self {
        CLNConf {
            fields: IndexMap::new(),
            includes: Vec::new(),
            path,
            create_if_missing,
        }
    }

    /// build a new instance of the parser.
    pub fn parser(&self) -> parser::Parser {
        parser::Parser::new(&self.path, self.create_if_missing)
    }

    pub fn add_conf(&mut self, key: &str, val: &str) -> Result<(), ParsingError> {
        if self.fields.contains_key(key) {
            let values = self.fields.get_mut(key).unwrap();
            for value in values.iter() {
                if val == value {
                    return Err(ParsingError {
                        core: 2,
                        cause: format!("field {key} with value {val} already present"),
                    });
                }
            }
            values.push(val.to_owned());
        } else {
            self.fields.insert(key.to_owned(), vec![val.to_owned()]);
        }
        Ok(())
    }

    /// Get a unique field with the specified key, if there are multiple definition
    /// the function return an error.
    ///
    /// In the case of multiple definition of the same key you would like to use `get_confs`.
    pub fn get_conf(&self, key: &str) -> Result<Option<String>, ParsingError> {
        let mut results = vec![];
        if let Some(fields) = self.fields.get(key) {
            results.append(&mut fields.clone());
        }
        for include in &self.includes {
            let fields = include.get_confs(key);
            if !fields.is_empty() {
                results.append(&mut fields.clone());
            }
        }
        if results.is_empty() {
            return Ok(None);
        }

        if results.len() > 1 {
            return Err(ParsingError {
                core: 1,
                cause: "mutiple field with the `{key}`".to_owned(),
            });
        }
        Ok(Some(results.first().unwrap().clone()))
    }

    /// Return a list of values with the specified key, if no
    /// item is found, return an empity vector.
    pub fn get_confs(&self, key: &str) -> Vec<String> {
        let mut results = vec![];
        if let Some(fields) = self.fields.get(key) {
            results.append(&mut fields.clone());
        }
        for include in &self.includes {
            let fields = include.get_confs(key);
            if !fields.is_empty() {
                results.append(&mut fields.clone());
            }
        }
        results
    }

    pub fn add_subconf(&mut self, conf: CLNConf) -> Result<(), ParsingError> {
        for subconf in &self.includes {
            if conf.path == subconf.path {
                return Err(ParsingError {
                    core: 2,
                    cause: format!("duplicate include {}", conf.path),
                });
            }
        }
        self.includes.push(conf.into());
        Ok(())
    }

    pub fn rm_conf(&mut self, key: &str, val: Option<&str>) -> Result<(), ParsingError> {
        if self.fields.contains_key(key) {
            match val {
                Some(val) => {
                    let values = self.fields.get_mut(key).unwrap();
                    if let Some(index) = values.iter().position(|x| x == val) {
                        values.remove(index);
                    } else {
                        return Err(ParsingError {
                            core: 2,
                            cause: format!("field {key} with value {val} not found"),
                        });
                    }
                }
                None => {
                    self.fields.remove_entry(key);
                }
            }
        } else {
            return Err(ParsingError {
                core: 2,
                cause: format!("field with `{key}` not present"),
            });
        }
        Ok(())
    }

    pub fn rm_subconf(&mut self, path: &str) -> Result<(), ParsingError> {
        if let Some(index) = self.includes.iter().position(|x| x.path == path) {
            self.includes.remove(index);
        } else {
            return Err(ParsingError {
                core: 2,
                cause: format!("include {path} not found"),
            });
        }
        Ok(())
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
        for field in self.fields.keys() {
            let values = self.fields.get(field).unwrap();
            if field.starts_with("comment") {
                let value = values.first().unwrap().as_str();
                content += &format!("{value}\n");
                continue;
            }
            for value in values {
                if value.is_empty() {
                    content += format!("{field}\n").as_str();
                    continue;
                }
                content += format!("{field}={value}\n").as_str();
            }
        }

        for include in &self.includes {
            content += format!("include {}\n", include.path).as_str();
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
        format!("{dir}/conf-{nanos}")
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
        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.fields.keys().len(), 2);

        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn subconf_add_rm() {
        let path = build_file("plugin=foo\nnetwork=bitcoin");
        assert!(path.is_ok());
        let path = path.unwrap();
        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok(), "{:#?}", result);
        assert_eq!(conf.fields.keys().len(), 2);
        assert_eq!(conf.includes.len(), 0);

        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        let subconf = CLNConf::new("/some/path".to_string(), false);
        assert!(conf.add_subconf(subconf).is_ok());
        assert_eq!(conf.includes.len(), 1);

        assert!(conf.rm_subconf("/some/path").is_ok());
        assert_eq!(conf.includes.len(), 0);

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_one() {
        let path = get_conf_path();
        let mut conf = CLNConf::new(path.to_string(), false);
        conf.add_conf("plugin", "/some/path").unwrap();
        conf.add_conf("network", "bitcoin").unwrap();
        let result = conf.flush();
        assert!(result.is_ok());

        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.fields.keys().len(), 2);
        println!("{conf:?}");
        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_two() {
        let path = get_conf_path();
        let mut conf = CLNConf::new(path.to_string(), false);
        conf.add_conf("plugin", "/some/path").unwrap();
        conf.add_conf("plugin", "foo").unwrap();
        conf.add_conf("network", "bitcoin").unwrap();
        let result = conf.flush();
        assert!(result.is_ok());

        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.fields.get("plugin").unwrap().len(), 2);
        println!("{conf:?}");
        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_three() {
        let path = get_conf_path();
        let mut conf = CLNConf::new(path.to_string(), false);
        conf.add_conf("network", "bitcoin").unwrap();
        conf.add_conf("plugin", "/some/path").unwrap();
        conf.add_conf("plugin", "/some/other/path").unwrap();
        conf.rm_conf("plugin", None).unwrap();
        let result = conf.flush();
        assert!(result.is_ok());

        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.fields.keys().len(), 1);
        println!("{conf:?}");
        assert!(!conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_four() {
        let path = get_conf_path();
        let mut conf = CLNConf::new(path.to_string(), false);
        conf.add_conf("network", "bitcoin").unwrap();
        conf.add_conf("plugin", "/some/path").unwrap();
        conf.add_conf("plugin", "/some/other/path").unwrap();
        conf.rm_conf("plugin", Some("/some/other/path")).unwrap();
        let result = conf.flush();
        assert!(result.is_ok());

        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        assert_eq!(conf.fields.keys().len(), 2);
        println!("{conf:?}");
        assert!(conf
            .fields
            .get("plugin")
            .as_ref()
            .map(|&s| s.contains(&"/some/path".to_string()))
            .unwrap_or(false));
        assert!(!conf
            .fields
            .get("plugin")
            .as_ref()
            .map(|&s| s.contains(&"/some/other/path".to_string()))
            .unwrap_or(false));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_with_comments() {
        let path = build_file("# this is just a commit\nplugin=foo\nnetwork=bitcoin");
        assert!(path.is_ok());
        let path = path.unwrap();
        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok());
        // subtract the comment item
        assert_eq!(conf.fields.keys().len() - 1, 2);

        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }

    #[test]
    fn flush_conf_with_includes() {
        let subpath = get_conf_path();
        let conf = CLNConf::new(subpath.clone(), false);
        assert!(conf.flush().is_ok());

        let path = build_file(
            format!("# this is just a commit\nplugin=foo\nnetwork=bitcoin\ninclude {subpath}")
                .as_str(),
        );
        assert!(path.is_ok(), "{}", format!("{path:?}"));
        let path = path.unwrap();
        let mut conf = CLNConf::new(path.to_string(), false);
        let result = conf.parse();
        assert!(result.is_ok(), "{}", result.unwrap_err().cause);
        // subtract the comment item
        assert_eq!(conf.fields.keys().len() - 1, 2);

        assert!(conf.fields.contains_key("plugin"));
        assert!(conf.fields.contains_key("network"));

        cleanup_file(path.as_str());
    }
}
