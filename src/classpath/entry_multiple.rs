use crate::classpath::entry::{new_entry, Entry, PATH_SEPARATOR};
use std::fmt;
use std::fmt::Formatter;

pub struct EntryMultiple {
    entries: Vec<Box<dyn Entry>>,
}

impl EntryMultiple {
    pub fn new(path_list: &str) -> Self {
        let path_list = path_list.split(PATH_SEPARATOR);
        let mut entries = vec![];
        for path in path_list {
            entries.push(new_entry(path))
        }
        Self { entries }
    }
}

impl Entry for EntryMultiple {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &mut self.entries {
            match entry.read_class(class_name) {
                Ok(data) => return Ok(data),
                Err(_err) => {}
            }
        }
        Err(format!("{} not fund", class_name))
    }
}

impl fmt::Display for EntryMultiple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}
