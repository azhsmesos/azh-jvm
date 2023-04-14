pub mod class_reader;
pub mod constant_pool;

use crate::classfile::constant_pool::ConstantPool;
use crate::types::RcRefCell;

/// ClassFile class文件结构体
/// 用来解析从class文件中读取的二进制数据
/// magic number：用于标识该文件是否为Java class文件，固定值为0xCAFEBABE。
/// version：描述该class文件的Java版本信息。
/// 常量池（Constant Pool）：保存了类中使用到的常量、变量、方法和类信息等。
/// 类访问标志（Access Flags）：用于描述类或接口的访问控制权限和特征。
/// This Class：用于表示当前类或接口在常量池中的索引值。
/// Super Class：用于表示当前类的父类在常量池中的索引值。
/// Interfaces：用于表示该类实现了哪些接口。
/// Fields：用于描述该类定义的字段信息，包括字段名称、类型及修饰符等。
/// Methods：用于描述该类定义的方法信息，包括方法名称、返回值类型、参数列表及修饰符等。
/// Attributes：用于描述类、字段或方法级别的附加信息，如注解、源码行号等。
pub struct ClassFile {
    sub_version: u16,                       // 次版本号
    major_version: u16,                     // 主版本号
    constant_pool: RcRefCell<ConstantPool>, // 常量池
}
