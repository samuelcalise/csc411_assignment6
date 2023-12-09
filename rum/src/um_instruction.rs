//use bitpack::bitpack::{getu};

#[derive(Debug)]
///Structure Instruction
///
///This structure has the opcode, a, b, c, and the value of the `u32` bit word
///from runtime.

pub struct Instruction {
    pub opcode: Opcode,
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub value: Option<u32>
}

#[derive(Debug, PartialEq)]
///Enum Opcode
///
///The `enum` used will have all the instruction functions necessary to emulate the rum machine
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Err
}

///Function: `get_opcode(instruction: u64) -> Opcode`
///
///This function is intended to return the `Opcode` in order to call the
///proper rum function during runtime.
pub fn get_opcode(instruction: u64) -> Opcode {

    //getting the opcode number form the u32 bit word
    let opcode_num = getu(instruction.try_into().unwrap(), 28, 4);

    if opcode_num == 0{
        Opcode::CMov
    }
    else if opcode_num == 1{
        Opcode::Load
    }
    else if opcode_num == 2{
        Opcode::Store
    }
    else if opcode_num == 3{
        Opcode::Add
    }
    else if opcode_num == 4{
        Opcode::Mul
    }
    else if opcode_num == 5{
        Opcode::Div
    }
    else if opcode_num == 6{
        Opcode::Nand
    }
    else if opcode_num == 7{
        Opcode::Halt
    }
    else if opcode_num == 8{
        Opcode::MapSegment
    }
    else if opcode_num == 9{
        Opcode::UnmapSegment
    }
    else if opcode_num == 10{
        Opcode::Output
    }
    else if opcode_num == 11{
        Opcode::Input
    }
    else if opcode_num == 12{
        Opcode::LoadProgram
    }
    else if opcode_num == 13{
        Opcode::LoadValue
    }
    else{
        Opcode::Err
    }
}

#[inline]
pub fn getu(word: u32, lsb: u32, width: u32) -> u32 {
    /*  //left shifting to get the most  |   //After the left shift, then
        //significant bit position in    |   //right shift off the uneeded
        //the unsigned value             |   //bits from the initial left shift */
        //eprintln!("{:032b} this is the word {},{}", word, width, lsb);
        return (word << (32 - width - lsb)) >> (32 - width)
    }

///Function: `get_a_bit(some_instruction: u64, opcode: &Opcode) -> u32`
///
///This function is intended to get and return the value of `A` in the instruction.
pub fn get_a_bit(some_instruction: u64, opcode: &Opcode) -> u32 {
    if *opcode == Opcode::LoadValue{
        return getu(some_instruction.try_into().unwrap(), 25, 3);
    }
    else{
        return getu(some_instruction.try_into().unwrap(), 6, 3);
    }
}

///Function: `get_b_bit(some_instruction: u64, opcode: &Opcode) -> u32`
///
///This function is intended to get and return the value of `B` in the instruction.
pub fn get_b_bit(some_instruction: u64, opcode: &Opcode) -> Option<u32> {
    if *opcode == Opcode::LoadValue{
        return None;
    }
    else{
        return Some(getu(some_instruction.try_into().unwrap(), 3, 3));
    }

}

///Function: `get_c_bit(some_instruction: u64, opcode: &Opcode) -> u32`
///
///This function is intended to get and return the value of `C` in the instruction.
pub fn get_c_bit(some_instruction: u32, opcode: &Opcode) -> Option<u32> {
    if *opcode == Opcode::LoadValue{
        return None
    }
    else{
        return Some(getu(some_instruction, 0, 3));
    }
    
}

///Function: `get_value(some_instruction: u64, opcode: &Opcode) -> Option<u32>`
///
///This function is intended to get and return the value from the instruction.
#[inline]
pub fn get_value(some_instruction: u32, opcode: &Opcode) -> Option<u32> {
        if *opcode == Opcode::LoadValue{
            return Some(getu(some_instruction, 0, 25));
        }
        else{
            return None
        }
}

//Instruction Implementation
impl Instruction {

    ///Function: `new(instruction: u32) -> Instruction`
    ///
    ///This function is intended to initialize an `Instruction` and return
    ///its structure data attributes.
    pub fn new(instruction: u32) -> Instruction {
        let opcode = get_opcode(instruction.into());
        let a = get_a_bit(instruction.into(), &opcode);
        let b = get_b_bit(instruction.into(), &opcode);
        let c = get_c_bit(instruction, &opcode);
        let value = get_value(instruction, &opcode);

        Instruction {
            opcode,
            a,
            b,
            c,
            value
        }
    }
}