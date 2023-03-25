use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;
use crate::classpath::entry::{Entry, get_absolute_path};

pub struct EntryCompress {
    abs_path: String,
    compress_archive: ZipArchive<File>
}

impl EntryCompress {

    pub fn new(path: &str) -> Self {
        let abs_path = get_absolute_path(path);
        let path = Path::new(&abs_path);
        let compress_file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("couldn't open {}: {}", &path.display(), err.to_string()),
        };

        Self {
            abs_path,
            compress_archive: ZipArchive::new(compress_file).unwrap(),
        }
    }
}

impl Entry for EntryCompress {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let archive = &mut self.compress_archive;
        let mut file = match archive.by_name(&class_name) {
            Ok(file) => file,
            Err(err) => return Err(format!("{} not found: {}", class_name, err.to_string())),
        };
        let mut vec: Vec<u8> = vec![];
        file.read_to_end(&mut vec).map_err(|err| err.to_string())?;
        Ok(vec)
    }
}

impl fmt::Display for EntryCompress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}