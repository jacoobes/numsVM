use crate::bytecode::{chunk::Chunk};

use super::stack::Stack;

pub struct VM {
    bytecode: Chunk,
    stack: Stack,
}

enum VMState {
    Completed,
    Panic,
}

impl VM {
    pub fn new(bytecode: Chunk) -> VM {
        VM {
            bytecode,
            stack: Stack::new(),
        }
    }

    fn eval(&mut self) -> VMState {
        let mut instructions = self.bytecode.instructions.iter().peekable();
        while let Some(&ip) = instructions.peek() {
            match ip {
                //HALT
                0x00 => {
                    instructions.next();
                    return VMState::Completed;
                }
                //LOAD_CONST
                0x01 => {
                    instructions.next();
                    let loc_of_const = match instructions.next() {
                        Some(c) => *c as usize,
                        None => return VMState::Panic,
                    };
                    self.stack
                        .push(self.bytecode.constants[loc_of_const]);
                }
                //NEGATE
                0x02 => {
                    let data = match self.stack.pop().negate() {
                        Ok(d) => d,
                        Err(e) => {
                            println!("{}",e);
                            return VMState::Panic;
                        },
                    }; 
                    self.stack.push(data);
                    instructions.next();
                }
                //ADD
                0x03 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left + right);
                    instructions.next();

                }
                //SUB
                0x04 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left - right);
                    instructions.next(); 
                }
                //MULT
                0x05 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left * right);
                    instructions.next(); 
                }
                //DIV 
                0x06 => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    self.stack.push(left / right);
                    instructions.next(); 
                }

                _ => {
                    panic!("unknown opcode!")
                }
            }
        }
        return VMState::Panic;
    }

    pub fn exec(&mut self) {
        //tokenize

        //parse


        self.eval();
        self.debug();
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
