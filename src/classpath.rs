use crate::classpath::entry::{Entry, new_entry};
use std::{env, fmt};
use std::fmt::Formatter;
use std::path::Path;
use tracing::{error};
use crate::classpath::entry_wildcard::EntryWildcard;
use crate::util::string_utils::{is_blank, is_not_blank};

pub mod entry;
pub mod entry_compress;
pub mod entry_dir;
pub mod entry_multiple;
pub mod entry_wildcard;

pub struct Classpath {
    boot_classpath: Box<dyn Entry>,
    ext_classpath: Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl Classpath {
    pub fn parse(jre_option: &str, cp_option: &str) -> Self {
        let boot_classpath = Classpath::parse_boot_classpath(jre_option);
        let ext_classpath = Classpath::parse_ext_classpath(jre_option);
        let user_classpath = Classpath::parse_user_classpath(cp_option);
        Self {
            boot_classpath,
            ext_classpath,
            user_classpath,
        }
    }


    fn parse_boot_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = Classpath::get_jre_path(jre_option);
        let path = Path::new(&jre_dir).join("lib").join("*");
        let jre_lib_path = path.to_str().unwrap();
        Box::new(EntryWildcard::new(jre_lib_path))
    }

    fn parse_ext_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = Classpath::get_jre_path(jre_option);
        let path = Path::new(&jre_dir).join("lib").join("ext").join("*");
        let jre_ext_path = path.to_str().unwrap();
        Box::new(EntryWildcard::new(jre_ext_path))
    }

    fn parse_user_classpath(cp_option: &str) -> Box<dyn Entry> {
        let mut classpath = cp_option;
        if is_blank(classpath) {
            classpath = ".";
        }
        new_entry(classpath)
    }

    fn get_jre_path(jre_option: &str) -> String {
        if is_not_blank(jre_option) {
            let jre_dir = Path::new(jre_option);
            if jre_dir.exists() {
                // 直接用用户的jre作为jre目录
                return jre_option.to_string();
            }
        }

        let jre_dir = Path::new("./jre");
        if jre_dir.exists() {
            return "./jre".to_string();
        }

        match env::var("JAVA_HOME") {
            Ok(java_home) => {
                if is_not_blank(&java_home) {
                    return Path::new(&java_home).join("jre").to_str().unwrap().to_string();
                }
            },
            Err(_err) => {error!("java environment variables were not obtained")}
        }

       panic!("{}", "Can't find jre folder!!!")
    }
}

impl Entry for Classpath {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let class = class_name.to_string() + ".class";
        return match self.boot_classpath.read_class(class.as_str()) {
            Ok(data) => Ok(data),
            Err(_err) => {
                match self.ext_classpath.read_class(&class) {
                    Ok(data) => Ok(data),
                    Err(_err) => {
                        match self.user_classpath.read_class(&class) {
                            Ok(data) => Ok(data),
                            Err(err) => return Err(err),
                        }
                    }
                }
            }
        }

    }
}

impl fmt::Display for Classpath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "user_classpath: {}, ext_classpath: {}, boot_classpath: {}",
            self.user_classpath, self.ext_classpath, self.boot_classpath
        )
    }
}
