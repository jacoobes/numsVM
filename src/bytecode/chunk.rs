pub struct Chunk {
    instruction : Vec<u8>
}
#[derive(Debug)]
pub enum OperatorCode {
    HALT = 0x0000
}

impl Chunk {
    
    pub fn new() -> Self {
        Chunk {
          instruction : Vec::new() 
        }
    }

    pub fn write(&mut self, op_code: &OperatorCode) {
        match op_code {
            OperatorCode::HALT => self.instruction.push(Chunk::op_to_u8(op_code))
        }     
    }

    pub fn op_to_u8(op_code: &OperatorCode) -> u8 {
            match op_code {
                OperatorCode::HALT => OperatorCode::HALT as u8 
            }
    }
}

impl core::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "======= Chunk =======\n").unwrap();
        match self.instruction[0] {
            0 => write!(f, "{}", format!("0x{:#04}      {:?}", 0x0000, OperatorCode::HALT)),
            _ => panic!("Unknown opcode!")
        }
    }
}