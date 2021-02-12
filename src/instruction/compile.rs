use super::util::{
    AddressingMode,
    Instruction,
    Mnemonic,
    Operand,
    change_endianness,
    split_word,
    make_word,
};
use super::instruction_table::MNEMONICS;

fn decode_mnemonic(ocode: &u8) -> Option<&Mnemonic> {
    for mnem in MNEMONICS.iter() {
        if mnem.opcode == *ocode {
            return Some(mnem);
        }
    }
    return None;
}

fn encode_operand(op: &Operand) -> Vec<u8> {
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

pub fn assemble(ins: &Instruction) -> Vec<u8> {
    let mut ret: Vec<u8>  = Vec::new();
    
    ret.push(ins.opcode);
    ret.append(
        &mut encode_operand(
            &ins.operand
        )
    );
    
    return ret;
}

fn addr_mode_to_oprnd_size(mode: AddressingMode) -> usize {
    match mode {
        AddressingMode::Absolute | AddressingMode::AbsoluteIndexedX | 
        AddressingMode::Indirect | AddressingMode::AbsoluteIndexedY => 2,
        AddressingMode::Relative | AddressingMode::IndirectIndexed | AddressingMode::IndexedIndirect |
        AddressingMode::ZeroPage | AddressingMode::ZeroPageIndexedX | AddressingMode::ZeroPageIndexedY => 1,
        _ => 0
    }
}

fn get_operand(bytes: &mut Vec<u8>, mode: AddressingMode) -> Option<Operand> {
    match addr_mode_to_oprnd_size(mode) {
        0 => {
            return Some(
                Operand::Null
            );
        },
        1 => {
            return Some(
                Operand::Byte(
                    bytes.pop()?
                )
            );
        },
        2 => {
            let (low, high): (u8,u8);
            low = bytes.pop()?;
            high = bytes.pop()?;

            return Some(
                Operand::Word(
                    make_word(&low, &high)
                )
            )
        }
        _ => { return None; }
    }
}

pub fn disassemble(bytes: &mut Vec<u8>) -> Option<Instruction> {
    let opcode = bytes.pop()?;
    let mnem = decode_mnemonic(&opcode)?;
    let operand = get_operand(bytes, mnem.addr_mode)?;

    Some(
        Instruction {
            opcode: opcode,
            operand: operand
        }
    )
}
/*
pub fn compile(code: &str) -> Vec<Instruction> {
    let ret: Vec<Instruction> = Vec::new();
    return ret;
}

pub fn decompile(ins: &Vec<Instruction>) -> String {
    let ret = String::new();
    return ret;
}*/