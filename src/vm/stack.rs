use std::slice::Iter;

use crate::bytecode::data::Data;

///abstraction over a vec of data values
/// supports basic pop, push, length, and iterator methods
pub struct Stack {
  stack : Vec<Data>  
}


impl Stack {
  
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }
    pub fn pop(&mut self) -> Data {  self.stack.pop().expect("No value to pop off the stack!") }
    pub fn push(&mut self, data: Data)  {  self.stack.push(data) }
    pub fn len(&self) -> usize { self.stack.len() }
    pub fn iter(&self) -> Iter<Data> { self.stack.iter() }
}

