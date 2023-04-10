

pub trait ConstantInfo {

}


pub struct ConstantPool {
    pub infos: Vec<Option<Box<dyn ConstantInfo>>>
}