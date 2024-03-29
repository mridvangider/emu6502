use super::super::util::{
    AddressingMode,
    Mnemonic,
    Operand,
    change_endianness,
    split_word,
    make_word,
    safe_remove,
};
use super::instruction_table::MNEMONICS;
use crate::errors::*;

/// Given a byte, returns the mnemonic with corresponding opcode.
pub fn decode_mnemonic<'a>(ocode: u8) -> Result<&'a Mnemonic, Err> {
    for mnem in MNEMONICS.iter() {
        if mnem.opcode == ocode {
            return Ok(mnem);
        }
    }
    return Err(ERR_INVALID_OPCDOE);
}

/// Turns an operand into a sequence of bytes. If the operand is 1-byte long,
/// then returns vector with that byte. If the operand is 2-byte long,
/// then returns vector with those 2 bytes in little endian format.
pub fn encode_operand(op: &Operand) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    
    match op {
        Operand::Byte(b) => { ret.push(*b); },
        Operand::Word(w) => { ret.append(
                &mut (split_word(
                    &change_endianness(w)
                ).to_vec())
            ); 
        }
        _ => {}
    }
    
    return ret;
}

/// For each addressing mode, returns the requiered the operand size in bytes.
pub fn addr_mode_to_oprnd_size(mode: AddressingMode) -> usize {
    match mode {
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | 
        AddressingMode::Indirect | AddressingMode::AbsoluteIndexedY => 2,
        AddressingMode::Relative | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::ZeroPageIndexedY => 1,
        _ => 0
    }
}

/// From a string of bytes, removes an operand from the beginning and returns it.
pub fn get_operand(bytes: &mut Vec<u8>, mode: AddressingMode) -> Result<Operand, Err> {
    match addr_mode_to_oprnd_size(mode) {
        0 => {
            return Ok(
                Operand::Null
            );
        },
        1 => {
            return Ok(
                Operand::Byte(
                    safe_remove(bytes, 1)?
                )
            );
        },
        2 => {
            let (low, high): (u8,u8);
            low = safe_remove(bytes, 1)?;
            high = safe_remove(bytes, 1)?;

            return Ok(
                Operand::Word(
                    make_word(&low, &high)
                )
            )
        }
        _ => { return Err(ERR_NO_VALID_OPERAND); }
    }
}

/*
/// Finds the corresponding Mnemonic from the given string.
pub fn find_mnemonic_by_name<'a> (name : &str) -> Result<&'a Mnemonic, Err> {
    for m in MNEMONICS.iter() {
        if name == m.name {
            return Ok(m);
        }
    }

    return Err(ERR_MNEMONIC_NOT_FOUND);
}

/// Turns a string into an operand.
pub fn process_operand(op : &str) -> Result<Operand, Err> {
    
}

pub fn compile_line(code: &str) -> Result<Instruction,Err> {
    match code.lines().next() {
        Some(line) => {
            let words : Vec<&str> = line.split(' ').collect();
            let name = words[0].trim().to_uppercase();
            let mnem = find_mnemonic_by_name(&name)?;
    
            if mnem.addr_mode == AddressingMode::Implied || mnem.addr_mode == AddressingMode::Accumulator {
                if words.len() == 1 {
                    return Ok(
                        Instruction {
                            mnemonic: mnem,
                            operand: Operand::Null,
                        }
                    )
                } else {
                    return Err(ERR_WRONG_OPERAND_COUNT);
                }
            } else {
                if words.len() == 2 {
                    return Ok(
                        Instruction {
                            mnemonic: mnem,
                            operand: process_operand(
                                words[1].to_ascii_uppercase().as_str(), 
                                mnem.addr_mode
                            )?
                        }
                    )

                } else {
                    return Err(ERR_WRONG_OPERAND_COUNT);
                }
            }
        },
        None => Err(ERR_INVALID_LINE)
    }
}

pub fn decompile(ins: &Vec<Instruction>) -> String {
    let ret = String::new();
    return ret;
}*/