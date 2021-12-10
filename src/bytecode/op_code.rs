#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum OpCode {
    HALT        = 0x00,
    LOAD_CONST  = 0x01,
    NEGATE      = 0x02,
    ADD         = 0x03,
    SUB         = 0x04,
    MUL         = 0x05,
    DIV         = 0x06
}
