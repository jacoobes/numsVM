use std::borrow::Cow;
use crate::bytecode::{chunk::Chunk, disassemble::disassemble_chunk};

use super::stack::Stack;

pub struct NumsVM {
    bytecode: Chunk,
    stack: Stack,
}

pub enum VMState {
    Completed,
    Panic,
}

impl NumsVM {
    pub fn new(bytecode: Chunk) -> NumsVM {
        NumsVM {
            bytecode,
            stack: Stack::new(),
        }
    }

    pub fn eval<'a> (&mut self, print_chunk: bool) -> Result<(), Cow<'a, str>> {
        let instructions =  &mut self.bytecode.instructions.iter();
        let constant_pool =  &self.bytecode.constants;
        while let Some(ip) = instructions.next() {
            match ip {
                //HALT
                0x00 => {
                    println!("HALT; program completed");
                    return Ok(());
                },
                //LOAD_CONST
                0x01 => {
                    let loc_const = match instructions.next() {
                        Some(location) => *location,
                        None => return Err(Cow::Borrowed("Expected a constant index after LOAD_CONST, found None")),
                    };
                    let const_data = match constant_pool.get(loc_const as usize) {
                        Some(a) => a.clone(),
                        None => {
                            println!("{:?}", disassemble_chunk(&self.bytecode));
                            return Err(Cow::Owned(format!("Expected constant at loc {}", loc_const)))
                        }   
                    };
                    self.stack.push(const_data);
                },
                //NEGATE
                0x02 => {
                   let data = self.stack.pop().negate()?; 
                   self.stack.push(data) 
                },
                //ADD
                0x03 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left + right);
                }
                //SUB
                0x04 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left - right);
                }
                //MULT
                0x05 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left * right);
                }
                //DIV
                0x06 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left / right);
                },
                _ => ()
            }
            
        };
        if print_chunk { self.debug() }
        Ok(())
    }

    pub fn debug(&mut self) {
        if self.stack.len() == 0 {
            println!("Stack is empty!")
        } else {
            println!("                    ");
            for value in self.stack.iter() {
                println!("[ {:?} ]", value)
            }
        }
    }
}
