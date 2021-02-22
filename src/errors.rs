pub type Err = u8;

pub const ERR_STACK_FULL : Err = 1;
pub const ERR_STACK_EMPTY : Err = 2;
pub const ERR_OPERAND_SIZE_INVALID : Err = 3;
pub const ERR_PARSE_ERROR : Err = 4;
pub const ERR_OPERAND_WRONG_FORMAT : Err = 5;
pub const ERR_WRONG_OPERAND_COUNT : Err = 6;
pub const ERR_INVALID_LINE : Err = 7;
pub const ERR_MNEMONIC_NOT_FOUND : Err = 8;
pub const ERR_NO_VALID_OPERAND : Err = 9;
pub const ERR_INVALID_INDEX : Err = 10;