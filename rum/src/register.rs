#[derive(Clone, Debug)]

///Structure: Register
///
///This structure is be a vector of u32 values which
///is intended to hold the register addresses during 
//runtime
pub struct Register {
    vec_registers: Vec<u32>,

}
//Register Implementation
impl Register {

    ///Function: new() -> Register
    ///
    ///This function is intended to initialize a register during runtime which has
    ///only 8 0_u32 values representing a blank set of registers
    pub fn new() -> Register {
        Register {
            vec_registers: vec![0; 8],
        }
    }

    ///Function: get_register_value(&self, register: usize) -> u32
    ///
    ///The getter function is intended to return the value
    ///at some `register` addresses in the `Register` structure
    ///vector. Then the getter function will return the value
    ///at the register's address as a u32 value.
    pub fn get_register_value(&self, register: usize) -> u32 {

        let reference = unsafe {
            self.vec_registers.get_unchecked(register)
        };
    
        *reference

        //self.vec_registers[register]
    }    

    ///Function: set_register_value(&mut self, register: usize) -> u32
    ///
    ///The setter function is intneded to recieve a register address
    ///and value which will be set within the `Register` vector at the
    ///address `register` with the passed in `value`.
    pub fn set_register_value(&mut self, register: usize, value: u32) {
        self.vec_registers[register] = value
    }
}