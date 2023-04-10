pub mod constant_pool;

use crate::classfile::constant_pool::ConstantPool;
use crate::types::RcRefCell;

/// ClassFile class文件结构体
/// 用来解析从class文件中读取的二进制数据
pub struct ClassFile {

    sub_version: u16, // 次版本号
    major_version: u16,  // 主版本号
    constant_pool: RcRefCell<ConstantPool>  // 常量池
 }