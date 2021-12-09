#[derive(Debug, Clone, Copy,)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum OpCode {
    HALT       = 0x00,
    LOAD_CONST = 0x01
}



