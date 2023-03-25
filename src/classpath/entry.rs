use std::fmt;
use std::path::Path;
use tracing::error;
use mockall::*;
use crate::classpath::entry_dir::EntryDir;


/// mac 系统分割是 ':'
/// windows 系统分割是 '\'
/// linux 系统分割是 '/'
#[cfg(not(windows))]
pub const PATH_SEPARATOR: char = ':';

#[cfg(windows)]
pub const path_separator: char = ';';

/// 目前有这么几种文件对象
/// -classpath file => 目录形式的类路径 entry_dir.rs
/// -classpath file1.jar:file2.jar:file3.jar  => 多个文件对象组成的类路径 entry_multiple.rs
/// -classpath file/*  => 以*结尾的路径 entry_wildcard.rs
/// -classpath file.jar => 压缩文件路径 entry_compression.rs
pub trait Entry: fmt::Display {

    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String>;
}

pub fn get_absolute_path(path: &str) -> String {
    let path = Path::new(path);
    match path.canonicalize() {
        Ok(pa) => pa.to_str().unwrap().to_string(),
        Err(err) => {
            error!("Exec canonicalize function error: {}", err);
            panic!("{}", err)
        },
    }
}

pub fn new_entry(path: &str) -> Box<dyn Entry> {

    // entry_multiple 多个文件

    // entry_wildcard *结尾

    // entry_compression 压缩文件


    Box::new(EntryDir::new(path))
}




#[cfg(test)]
mod tests {
    use std::io;
    use std::path::PathBuf;
    use mockall::{automock, mock};
    use crate::classpath::entry::get_absolute_path;



    #[test]
    fn test_get_absolute_path() {

        // struct Function{}
        // #[automock]
        // trait Functions {
        //     fn canonicalize() -> io::Result<PathBuf>;
        // }
        //
        // let mut mock = MockFunctons::new();
        let path: &str = "src/classpath/entry.rs";
        let absolute_path = get_absolute_path(path);
        assert_eq!(absolute_path, "/Users/zhaozhenhang/Desktop/test/rust/azh-jvm/src/classpath/entry.rs".to_string());
    }
}