use crate::classpath::entry::{get_absolute_path, Entry};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct EntryDir {
    abs_dir: String,
}

impl EntryDir {
    pub fn new(path: &str) -> Self {
        Self {
            abs_dir: get_absolute_path(path),
        }
    }
}

impl Entry for EntryDir {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let path = Path::new(&self.abs_dir);
        let new_path = path.join(class_name);
        let mut file = match File::open(new_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(format!("{} not found: {}", class_name, err.to_string()));
            }
        };

        let mut vec: Vec<u8> = vec![];
        file.read_to_end(&mut vec).map_err(|err| err.to_string())?;
        Ok(vec)
    }
}

impl fmt::Display for EntryDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abs_dir)
    }
}
