use std::ops;

use crate::Type::*;
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
    pub fn negate(&self) -> Result<Self, std::string::String> {
        match self.val {
            Type::F64(f) => Ok(Self { val: Type::F64(-f) }),
            Type::F32(f) => Ok(Self { val: Type::F32(-f) }),
            Type::I32(f) => Ok(Self { val: Type::I32(-f) }),
            Type::I64(f) => Ok(Self { val: Type::I64(-f) }),
            Type::Bool(_) => Err(std::string::String::from("Cannot negate a boolean")),
            Type::String(_) => Err(std::string::String::from("Cannot negate a string")),
            Type::Char(_) => Err(std::string::String::from("Cannot negate a char")),
        }
    }
}

impl Copy for Data {}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self { val: self.val.clone() }
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.val {
            F64(v) =>     write!(f,"({:>3} f64)", v),
            F32(v) =>     write!(f,"({:>3} f32)", v),
            I32(v) =>     write!(f,"({:>3} i32)", v),
            I64(v) =>     write!(f,"({:>3} i64)", v),
            Bool(v) =>   write!(f,"({:>3} bool)", v),
            String(v) => write!(f,"({:>3} string)", v),
            Char(v) =>   write!(f,"({:>3} char)", v),
        }
    }
}

///
/// This is just putting some crap together to make bin ops work,
///  There's probably a macro / more optimized way to do this
/// 

impl ops::Add<Data> for Data {
    type Output = Data;
    
    fn add(self, rhs: Data) -> Self::Output {
        match (self.val, rhs.val) {
            (F64(f1), F64(f2)) => Self { val : F64(f1 + f2)},
            (F32(f1), F32(f2)) => Self { val : F32(f1 + f2)},
            (I32(i1), I32(i2)) => Self { val : I32(i1 + i2)},
            (I64(i1), I64(i2)) => Self { val : I64(i1 + i2)},
            (other, other2)=> panic!("Cannot perform + on {:?} {:?}. Mismatch types!", other, other2)
        }
    }
}

impl ops::Sub<Data> for Data {
    type Output = Data;
    
    fn sub(self, rhs: Data) -> Self::Output {
        match (self.val, rhs.val) {
            (F64(f1), F64(f2)) => Self { val : F64(f1 - f2)},
            (F32(f1), F32(f2)) => Self { val : F32(f1 - f2)},
            (I32(i1), I32(i2)) => Self { val : I32(i1 - i2)},
            (I64(i1), I64(i2)) => Self { val : I64(i1 - i2)},
            (other, other2)=> panic!("Cannot perform + on {:?} {:?}. Mismatch types!", other, other2)
        }
    }
}

impl ops::Mul<Data> for Data {
    type Output = Data;
    
    fn mul(self, rhs: Data) -> Self::Output {
        match (self.val, rhs.val) {
            (F64(f1), F64(f2)) => Self { val : F64(f1 * f2)},
            (F32(f1), F32(f2)) => Self { val : F32(f1 * f2)},
            (I32(i1), I32(i2)) => Self { val : I32(i1 * i2)},
            (I64(i1), I64(i2)) => Self { val : I64(i1 * i2)},
            (other, other2)=> panic!("Cannot perform + on {:?} {:?}. Mismatch types!", other, other2)
        }
    }
}

impl ops::Div<Data> for Data {
    type Output = Data;
    
    fn div(self, rhs: Data) -> Self::Output {
        match (self.val, rhs.val) {
            (F64(f1), F64(f2)) => Self { val : F64(f1 / f2)},
            (F32(f1), F32(f2)) => Self { val : F32(f1 / f2)},
            (I32(i1), I32(i2)) => Self { val : I32(i1 / i2)},
            (I64(i1), I64(i2)) => Self { val : I64(i1 / i2)},
            (other, other2)=> panic!("Cannot perform + on {:?} {:?}. Mismatch types!", other, other2)
        }
    }
}