pub mod class_info;
pub mod constant_pool_tag;
pub mod number_info;
pub mod string_info;

use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::class_info::ConstantClassInfo;
use std::collections::HashMap;

pub trait ConstantInfo {
    fn read_info(&mut self, reader: &mut ClassReader);

    fn tag(&self) -> u8;
}

pub struct ConstantPool {
    pub infos: Vec<Option<Box<dyn ConstantInfo>>>,

    pub class_info_map: HashMap<u16, ConstantClassInfo>,
}

impl ConstantPool {
    // todo
}
