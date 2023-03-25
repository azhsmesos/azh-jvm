use crate::classpath::entry::Entry;

mod entry;
mod entry_dir;


pub struct Classpath {
    boot_classpath: Box<dyn Entry>,
    ext_classpath: Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl Classpath {

    pub fn parse(jre_option: &str, cp_option: &str) -> Self {

        // Self {
        //
        // }
        // todo
    }
}