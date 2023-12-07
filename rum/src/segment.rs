
use std::mem;
use crate::{um_instruction::Instruction};


#[derive(Debug, Clone)]
///Structure: Segment
///
///The structure will have many addresses and instructions during runtime and during testing.
///`addresses` is a vector of `u32` values at are determined from the `u32` word and the `instructions`
///are based on an adresses making the `instructions` a 2D vector of `u32` values. 
pub struct Segment {
    addresses: Vec<usize>,
    instructions: Vec<Vec<u32>>
}

impl Segment {

    ///Function: `new(some_instruction: &Vec<u32>) -> Segment`
    ///
    ///This function initializes a new `Segment` which has
    ///`addresses` and `instructions` which are vector respected to
    ///the passed `some_instruction` during runtime.
    pub fn new(some_instruction: &Vec<u32>) -> Segment
    {
        Segment{
            addresses: Vec::new(),
            instructions: vec![some_instruction.to_vec()]
        }
    }

    ///Function: `map_segment(& mut self, size: usize) -> usize`
    ///
    ///This function will returning an removed address from the vector of `addresses` that
    ///will be returned and replaces with a zero of all zero's based on an instruction's `size`.
    pub fn map_segment(& mut self, size: usize) -> usize
    {
        let zero_vec =  vec![0_u32; size];

        if self.addresses.is_empty()
        {
            self.instructions.push(zero_vec);


            self.instructions.len() - 1
        }
        else
        {
            let this_address = self.addresses.pop().unwrap();
            let _new_address = mem::replace(self.instructions.get_mut(this_address).unwrap(), zero_vec);


            this_address
        }
    }

    ///Function: `unmap_segment(& mut self, some_address: usize)`
    ///
    ///This function will be unmapping and replacing a value at `some_address` in the
    ///`instructions` vector.
    pub fn unmap_segment(& mut self, some_address: usize)
    {
        self.addresses.push(some_address);

        let _new_address = mem::replace(self.instructions.get_mut(some_address).unwrap(), Vec::new());
    }

    ///Function: `get_segment_value(&self, some_address: usize) -> Option<&Vec<u32>>`
    ///
    ///The helper function is designed to return the `instructions` of a certain segment
    ///at `some_address`.
    pub fn get_segment_value(&self, some_address: usize) -> Option<&Vec<u32>>
    {
        self.instructions.get(some_address)
    }

    ///Function: `find_instruction(&self, c: usize) -> Instruction`
    ///
    ///This function is intended to find the `Instruction` of the segment's opcode
    ///value that will have an intended `Instruction`.
    pub fn find_instruction(&self, c: usize) -> Instruction
    {
        match self.instructions.get(0){
            Some(segment) => Instruction::new(segment[c]),
            None => panic!("No more further instructions")
        }
    }

    ///Function: `set_segment_value(&mut self, some_address: usize, index: usize, value: u32)`
    ///
    ///The function will be recieving `some_address`, `index`, and `value` from the `u32` word
    ///during runtime. The function will obtain the `current_segment` at `some_address` that will
    ///replaced at the `current_segment`'s at `index` and will have a new `value` that is passed 
    ///into the function.
    pub fn set_segment_value(&mut self, some_address: usize, index: usize, value: u32)
    {
        let current_segment = self.instructions.get_mut(some_address).unwrap();

        let _new_segment = mem::replace(current_segment.get_mut(index).unwrap(), value);
    }

    ///Function: `insert_value(&mut self, some_address: usize)`
    ///
    ///This function will be inserting a segment at the `0` position of `instructions` that is a 
    ///`cloned_segment` of `some_address` to a newer segment.
    pub fn insert_value(&mut self, some_address: usize)
    {
        let cloned_segment = self.instructions.get(some_address).unwrap().clone();

        let _new_segment = mem::replace(self.instructions.get_mut(0).unwrap(), cloned_segment);
    }
}