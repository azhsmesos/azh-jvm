use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::constant_pool_tag::CONSTANT_CLASS;
use crate::classfile::constant_pool::{ConstantInfo, ConstantPool};
use crate::types::RcRefCell;

pub struct ConstantClassInfo {
    constant_pool: RcRefCell<ConstantPool>,
    class_index: u16,
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }
}

impl ConstantClassInfo {
    pub fn new(constant_pool: RcRefCell<ConstantPool>) -> Self {
        Self {
            constant_pool,
            class_index: 0,
        }
    }
}
