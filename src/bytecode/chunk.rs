use super::{data::Data, op_code::OpCode::{self, *}};
#[derive(Debug)]
pub struct Chunk {
    pub instructions: Vec<u8>,
    pub constants: Vec<Data>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
    
    /// writes bytes to a chunk.
    /// Associates opcode to its corresponding byte
    pub fn write(&mut self, op_code: &OpCode) {
        self.instructions.push(Chunk::op_to_u8(op_code));
    }
    /// All consts are located on a constants pool (a vec),
    pub fn write_const(&mut self, const_index: u8) {
        self.instructions.push(const_index)
    }
    pub fn op_to_u8(op_code: &OpCode) -> u8 { *op_code as u8 }
    /// adds a const to the constant pool and returning the index of where it is. ( the most recently pushed element)
    pub fn add_const(&mut self, val: crate::bytecode::data::Data) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }
}
/// higher order abstractions of previous implementations
impl Chunk {
    pub fn emit_const(&mut self, data : Data) {
        let loc_of_const =  self.add_const( data);
        self.write(&LOAD_CONST);
        self.write_const(loc_of_const)
     }
     
     pub fn emit_op(&mut self, code: &OpCode) {
         self.write(&code);
     }

}
