use crate::bytecode::data::Type::*;
use crate::bytecode::{chunk::Chunk, data::Data};

pub struct VM {
    bytecode: Chunk,
    ip: u8,
    stack: Vec<Data>,
}

enum State {
    Completed,
    Panic,
}

impl VM {
    pub fn new(bytecode: Chunk) -> VM {
        VM {
            bytecode,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn eval(&mut self) -> State {
        let mut instructions = self.bytecode.instructions.iter().peekable();
        while let Some(ip) = instructions.peek() {
            match ip {
                //HALT
                0x00 => {
                    instructions.next();
                    return State::Completed;
                }
                //LOAD_CONST
                0x01 => {
                    instructions.next();
                    let loc_of_const = match instructions.next() {
                        Some(c) => c,
                        None => return State::Panic,
                    };
                    self.stack
                        .push(self.bytecode.constants[*loc_of_const as usize]);
                }
                0x02 => {
                    let data = match self.stack.pop() {
                        Some(data) => data.negate(),
                        None => panic!("No value found to negate!"),
                    };
                    instructions.next();
                    self.stack.push(data.unwrap());
                }

                _ => {
                    panic!("unknown opcode!")
                }
            }
        }
        return State::Panic;
    }

    fn stack_eval(&mut self) -> State {
        let instructions = &self.bytecode.instructions;
        let constants = &self.bytecode.constants;
        loop {
            match self.ip {
                //HALT
                0x00 => {
                    println!("HALT");
                    self.ip += 1;
                    return State::Completed;
                }
                //LOAD_CONST
                0x01 => {
                    let loc_of_const = match instructions.get((self.ip as usize) + 1) {
                        Some(d) => *d as usize,
                        None => panic!("dfs"),
                    };
                    self.stack.push(constants[loc_of_const]);
                    self.ip += 2;
                }
                _ => {
                    panic!("unknown opcode!")
                }
            }
        }
    }

    pub fn exec(&mut self, stack: bool) {
        //tokenize

        //parse
        if stack {
            self.stack_eval();
        } else {
            self.eval();
        }
        self.debug();
    }

    pub fn debug(&mut self) {
        if self.stack.is_empty() {
            println!("Stack is empty!")
        } else {
            println!("                    ");
            for value in &self.stack {
                println!("[ {:?} ]", value)
            }
        }
    }
}
