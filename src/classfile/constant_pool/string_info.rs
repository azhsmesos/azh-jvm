use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::constant_pool_tag::CONSTANT_STRING;
use crate::classfile::constant_pool::{ConstantInfo, ConstantPool};
use crate::types::RcRefCell;

/// ConstantStringInfo string_index 指向常量池中该字符串字面量的索引值
pub struct ConstantStringInfo {
    constant_pool: RcRefCell<ConstantPool>,
    string_index: u16,
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.string_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_STRING
    }
}

impl ConstantStringInfo {
    pub fn new(constant_pool: RcRefCell<ConstantPool>) -> Self {
        Self {
            constant_pool,
            string_index: 0,
        }
    }
}
