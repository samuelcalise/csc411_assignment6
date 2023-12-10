use std::io::{stdin, Read};
use crate::{register::Register, segment::Segment, um_instruction::Instruction};
use std::io::stdout;
use std::io::Write;
#[derive(Debug, Clone)]

///Structure: Rum
///
///This structure has a `Segment` and `Register` which
///the `Segment` will have the instruction word during runtime
///and the `Register` will have the values and address of the 
///value assignments based on the given instructions.
pub struct Rum {
    segment: Segment,
    register: Register
}
//Rum Implementation
impl Rum {

    ///Function: `new(some_instruction: &Vec<u32>) -> Rum`
    ///
    ///This function is intended to initialize the `Rum`
    ///machine where the `segment` will have the instruction value
    ///and a new vector `register` will be initialized before 
    ///following the opcode functions.
    pub fn new(some_instruction: &Vec<u32>) -> Rum
    {
        Rum{

            segment: Segment::new(&some_instruction),
            register: Register::new()
        }
    }

    ///Function: `fn get_instruction(&self, c: usize) -> Instruction`
    ///
    ///This function is intended to get the instruction from a helper
    ///function. The function will return the found `Instruction` from the
    ///helper function `find_instruction`.
    pub fn get_instruction(&self, c: usize) -> Instruction {
       
        unsafe {
            self.segment.find_instruction(c)
        }
    }
    
    ///Function: `conditional_move(&mut self, some_instruction: Instruction)`
    ///
    ///The function `conditional_move` takes in the instruction `some_instruction`
    ///which has the values A (`a_bit`),B (`b_bit`), and C(c_bit) that are used 
    ///to conditionally move a value and its address based on a condition. 
    ///The function will determine whether the value at `register[c_bit]` is 0 or not.
    ///Based on this condition, the value of B will be moved to `register[A]`.
    pub fn conditional_move(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        if self.register.get_register_value(c_bit) != 0{
            
            let value = self.register.get_register_value(b_bit);

            self.register.set_register_value(a_bit, value);
        }
    }

    ///Function: `segment_load(&mut self, some_instruction: Instruction)`
    ///
    ///This function will load a value to from the `some_instruction` that
    ///will be assigned at address at the values of `b_bit` and `c_bit` which
    ///stores the found value of `b_bit` and `c_bit` in the register at the address
    ///of value `a_bit` in the current `register` during runtime.
    pub fn segment_load(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let this_address = self.register.get_register_value(b_bit) as usize;

        let vec = self.segment.get_segment_value(this_address).unwrap();

        let reg_index = self.register.get_register_value(c_bit) as usize;

        let value = vec[reg_index];

        self.register.set_register_value(a_bit, value);
    }

    ///Function: `segment_store(&mut self, instruction: Instruction)`
    ///
    ///The function will store the segmented instruction based on the values
    ///of `a_bit`, `b_bit`, and `c_bit`. Once the values are found within the
    ///object `Instruction` the segment will store a value from `c_bit` at the
    ///address (`a_bit`) at index (`b_bit`) within segment.
    pub fn segment_store(&mut self, some_instruction: Instruction) 
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;
        
        let this_address = self.register.get_register_value(a_bit) as usize;

        let index = self.register.get_register_value(b_bit) as usize;

        let value = self.register.get_register_value(c_bit);

        self.segment.set_segment_value(this_address, index, value);
    }

    ///Function: `addition(&mut self, some_instruction: Instruction)`
    ///
    ///The `addition` function will add the values of `b_bit` and `c_bit` at the 
    ///address of `a_bit` from the passed in `some_instruction`. The values 
    ///will be stored at the address of `a_bit` in the register during runtime.
    pub fn addition(&mut self, some_instruction: Instruction) {

        let (a_bit, b_bit, c_bit) = (
            
            some_instruction.a as usize,
            some_instruction.b.unwrap() as usize,
            some_instruction.c.unwrap() as usize,
        );
    
        let value = self.register.get_register_value(b_bit).wrapping_add(self.register.get_register_value(c_bit));
        self.register.set_register_value(a_bit, value);
    }
    

    ///Function: `multiplication(&mut self, some_instruction: Instruction)`
    ///
    ///The `multiplication` function will multiply the values of `b_bit` and `c_bit` at the 
    ///address of `a_bit` from the passed in `some_instruction`. The values 
    ///will be stored at the address of `a_bit` in the register during runtime.
    pub fn multiplication(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = self.register.get_register_value(b_bit).wrapping_mul(self.register.get_register_value(c_bit));

        self.register.set_register_value(a_bit, value);
    }

    ///Function: `division(&mut self, some_instruction: Instruction)`
    ///
    ///The `division` function will divide the values of `b_bit` and `c_bit` at the 
    ///address of `a_bit` from the passed in `some_instruction`. The values 
    ///will be stored at the address of `a_bit` in the register during runtime.
    pub fn division(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap();

        let c_bit = some_instruction.c.unwrap();

        let value = self.register.get_register_value(b_bit as usize).wrapping_div(self.register.get_register_value(c_bit as usize)) as u32;

        self.register.set_register_value(a_bit, value);
    }

    ///Function: `bit_nand(&mut self, some_instruction: Instruction)`
    ///
    ///The `bit_nand` function will compute the bit operator nand the values of `b_bit` and `c_bit` at the 
    ///address of `a_bit` from the passed in `some_instruction`. The values 
    ///will be stored at the address of `a_bit` in the register during runtime.
    pub fn bit_nand(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        let value = !(self.register.get_register_value(b_bit) & self.register.get_register_value(c_bit));

        self.register.set_register_value(a_bit, value);
    }

    ///Function: `map_segment(&mut self, some_instruction: Instruction)`
    ///
    ///The `map_segment` function will be reassigning the a segment to a new
    ///location in the `register`.
    pub fn map_segment(&mut self, some_instruction: Instruction) {

        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;
    
        let new_size = unsafe { self.register.get_register_value(c_bit) as usize };
    
        let new_address = unsafe { self.segment.map_segment(new_size) };
    
        unsafe { self.register.set_register_value(b_bit, new_address as u32) };
    }
    

    ///Function: `unmap_segment(&mut self, some_instruction: Instruction)`
    ///
    ///The `unmap_segment` function will be unmaping a segment of its location
    ///in the current `register`.
    pub fn unmap_segment(&mut self, some_instruction: Instruction) 
    {

        let c_bit = unsafe { some_instruction.c.unwrap() as usize };
    
        let this_address = unsafe { self.register.get_register_value(c_bit) as usize };
    
        unsafe {
            self.segment.unmap_segment(this_address);
        }
    }    

    ///Function: `output_program(&mut self, some_instruction: Instruction)`
    ///
    ///The function will be printing out the `char` values from a provided 
    ///program of values `[0-255]`.
    pub fn output_program(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        let c_value = self.register.get_register_value(c_bit);

        if c_value > 255
        {
            panic!("The value is outside of [0-255]")
        }

        print!("{}", char::from_u32(c_value).unwrap());
        stdout().flush().unwrap();
    }

    ///Function: `user_input(&mut self, some_instruction: Instruction)`
    ///
    ///This function is intended to handle user input during program runtime.
    pub fn user_input(&mut self, some_instruction: Instruction)
    {
        let c_bit = some_instruction.c.unwrap() as usize;

        //try to implment match statement when iteratoring through a std input in terminal**
        if let Some(Ok(value)) = stdin().bytes().next()
        {
            
            self.register.set_register_value(c_bit, value as u32);
        }
        else
        {
            self.register.set_register_value(c_bit, std::u32::MAX);
        }
    }

    ///Function: `load_program(&mut self, some_instruction: Instruction) -> usize`
    ///
    ///This function is intended to load a program which it may have to insert its
    ///value to `register[b_bit]` when the value equals 0. Otherwise, the function
    ///will return the value of `c_bit` from the instruction.
    pub fn load_program(&mut self, some_instruction: Instruction) -> usize
    {
        let b_bit = some_instruction.b.unwrap() as usize;

        let c_bit = some_instruction.c.unwrap() as usize;

        if self.register.get_register_value(b_bit) != 0
        {
            self.segment.insert_value(self.register.get_register_value(b_bit) as usize);
        }

        self.register.get_register_value(c_bit) as usize
    }

    ///Function: `load_value(&mut self, some_instruction: Instruction)`
    ///
    ///This function is intended to get set the `value` from `some_instruction` at the
    ///address of `a_bit` in the register during runtime.
    pub fn load_value(&mut self, some_instruction: Instruction)
    {
        let a_bit = some_instruction.a as usize;

        let value = some_instruction.value.unwrap();

        self.register.set_register_value(a_bit, value);
    }
}