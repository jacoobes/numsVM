#[derive(Debug)]
pub struct Data {
    pub val : Type
}
#[derive(Debug, Clone, Copy)]
pub enum Type {
    F64(f64),
    F32(f32),
    I32(i32),
    I64(i64),
    Bool(bool),
    String(&'static str),
    Char(char)
}

impl Data {
    pub fn new (typ : Type) -> Self {
        Self { val : typ  }
    } 
}

impl Copy for Data {

}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self { val: self.val.clone() }
    }
}