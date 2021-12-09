#[derive(Debug)]
pub struct Data {
    pub val: Type,
}
#[derive(Debug, Clone, Copy)]
pub enum Type {
    F64(f64),
    F32(f32),
    I32(i32),
    I64(i64),
    Bool(bool),
    String(&'static str),
    Char(char),
}

impl Data {
    pub fn new(typ: Type) -> Self {
        Self { val: typ }
    }
    pub fn negate(&self) -> Result<Self, String> {
        match self.val {
            Type::F64(f) => Ok(Self { val: Type::F64(-f) }),
            Type::F32(f) => Ok(Self { val: Type::F32(-f) }),
            Type::I32(f) => Ok(Self { val: Type::I32(-f) }),
            Type::I64(f) => Ok(Self { val: Type::I64(-f) }),
            Type::Bool(_) => Err(String::from("Cannot negate a boolean")),
            Type::String(_) => Err(String::from("Cannot negate a string")),
            Type::Char(_) => Err(String::from("Cannot negate a char")),
        }
    }
}

impl Copy for Data {}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
        }
    }
}
