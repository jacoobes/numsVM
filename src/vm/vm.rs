use crate::bytecode::{chunk::Chunk, data::Data};



pub struct VM {
    bytecode : Chunk,
    stack : Vec<Data>
}

enum State {
    Completed,
    Panic
}

impl VM {


    pub fn new (bytecode: Chunk) -> VM {
        
        VM {
            bytecode,
            stack: Vec::new()
        }
    }

    fn eval(&mut self) -> State {
        let mut instructions = self.bytecode.instructions.iter().peekable();
        while let Some(&ip) = instructions.peek() {
            match ip  {
                0x00 => {
                    println!("HALT");
                    instructions.next();
                    return State::Completed;
                },
                0x01 => {
                    instructions.next();
                    let loc_of_const = match instructions.next() {
                        Some(c) => c,
                        None => return State::Panic,
                    };                    
                    self.stack.push( self.bytecode.constants[*loc_of_const as usize]);
                }
                _ => {
                    panic!("unknown opcode!")
                }
            }
        };
        return State::Panic
    }

    fn pop(&mut self) -> Data {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("No value found!")
        }
    }

    pub fn exec(&mut self) {
        //tokenize

        //parse
        self.eval();
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