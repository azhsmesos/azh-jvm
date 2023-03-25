use std::{fmt, fs};
use crate::classpath::entry::{Entry, PATH_SEPARATOR};
use crate::classpath::entry_compress::EntryCompress;

pub struct EntryWildcard {
    entries: Vec<Box<dyn Entry>>,
}

impl EntryWildcard {

    pub fn new(path: &str) -> Self {
        let base_dir = &path[..path.len() - 1];
        let dir = match fs::read_dir(base_dir) {
            Ok(dir) => dir,
            Err(err) => panic!("couldn't open {}: {}", base_dir, err.to_string())
        };
        let convert = |entry| -> Box<dyn Entry> {
          Box::new(entry)
        };

        let mut entries = vec![];
        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                continue
            }
            let path_str = path.to_str().unwrap();
            if path_str.ends_with(".jar") || path_str.ends_with(".JAR") {
                let compress_entry = EntryCompress::new(&path.to_str().unwrap());
                entries.push(convert(compress_entry));
            }
        }

        Self {
            entries
        }
    }
}

impl Entry for EntryWildcard {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
       for entry in &mut self.entries {
            match entry.read_class(class_name) {
                Ok(data) => return Ok(data),
                Err(_err) => {},
            }
       }
        Err(format!("{} not fund", class_name))
    }
}

impl fmt::Display for EntryWildcard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}