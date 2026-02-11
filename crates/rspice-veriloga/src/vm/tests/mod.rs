use super::*;
use crate::codegen::{BytecodeProgram, Instruction, LookupTable};

fn make_program(instructions: Vec<Instruction>) -> BytecodeProgram {
    BytecodeProgram { instructions }
}

mod core_ops;
mod filters_and_events;
mod internal_nodes_and_logic;
mod state_and_limit;
mod tables_and_delay;
