use std::env;
use std::process;
use std::convert::TryInto;
pub mod rum;
pub mod segment;
pub mod register;
pub mod um_instruction;

//function take from past lab
pub fn load_instruction(input: Option<&str>) -> Vec<u32> 
{
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut buf = Vec::<u8>::new(); 
    raw_reader.read_to_end(&mut buf).unwrap();

    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap())) 
        .collect();
    
    instructions
}

fn main() 
{
    //Getting arguments from the command line
    let command_line: Vec<String> = env::args().collect();

    //File that will be used during runtime
    let command_file = &command_line[1];

    //Getting the u32bit instruction word
    let runtime_instruction = load_instruction(Some(command_file));

    //Initializing a 'rum' object to begin the insturction that 
    //is supposed to be emulated
    let mut rum = rum::Rum::new(&runtime_instruction);

    let mut instruction_count = 0;

    loop {
        let this_instruction = rum.get_instruction(instruction_count);
        instruction_count += 1;

        match this_instruction.opcode {
            um_instruction::Opcode::CMov => rum.conditional_move(this_instruction),
            um_instruction::Opcode::Load => rum.segment_load(this_instruction),
            um_instruction::Opcode::Store => rum.segment_store(this_instruction),
            um_instruction::Opcode::Add => rum.addition(this_instruction),
            um_instruction::Opcode::Mul => rum.multiplication(this_instruction),
            um_instruction::Opcode::Div => rum.division(this_instruction),
            um_instruction::Opcode::Nand => rum.bit_nand(this_instruction),
            um_instruction::Opcode::Halt => {
                //println!("The total number of instructions: {}", instruction_count);
                process::exit(0);
            }
            um_instruction::Opcode::MapSegment => rum.map_segment(this_instruction),
            um_instruction::Opcode::UnmapSegment => rum.unmap_segment(this_instruction),
            um_instruction::Opcode::Output => rum.output_program(this_instruction),
            um_instruction::Opcode::Input => rum.user_input(this_instruction),
            um_instruction::Opcode::LoadProgram => {
                instruction_count = rum.load_program(this_instruction);
            }
            um_instruction::Opcode::LoadValue => rum.load_value(this_instruction),
            um_instruction::Opcode::Err => panic!("Unknown opcode for instruction {:?}", this_instruction),
        }
    }
}