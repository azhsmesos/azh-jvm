use std::env::var;

/// ClassReader 读取Java class文件并解析其中的字节码信息。
/// 1. 读取class文件：ClassReader能够从输入流或者字节数组中读取Java class文件，并将其解析成内部数据结构。
/// 2. 解析常量池：ClassReader能够解析常量池中的各种常量类型，如字符串、类、字段等。
/// 3. 解析类信息：ClassReader能够解析类的访问标志、父类、接口、字段、方法等信息。
/// 4. 解析指令码信息：ClassReader能够解析方法中的字节码指令，包括操作码、操作数、跳转目标等信息.
pub struct ClassReader {
    data: Vec<u8>,
}

impl ClassReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read_u8(&mut self) -> u8 {
        let magic = self.data[0];
        self.data = self.data[1..].to_vec();
        magic
    }

    pub fn read_u16(&mut self) -> u16 {
        let magic = u16::from_be_bytes((&self.data[..2]).try_into().unwrap());
        self.data = self.data[2..].to_vec();
        magic
    }

    pub fn read_u32(&mut self) -> u32 {
        let magic = u32::from_be_bytes((&self.data[..4]).try_into().unwrap());
        self.data = self.data[4..].to_vec();
        magic
    }

    pub fn read_u64(&mut self) -> u64 {
        let magic = u64::from_be_bytes((&self.data[..8]).try_into().unwrap());
        self.data = self.data[8..].to_vec();
        magic
    }
}
