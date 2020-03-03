use crate::ir::*;
use crate::name::*;
use std::error::Error;
use std::io::BufWriter;
use std::fs::File;

pub fn compile_to_c(program: &IRProgram, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    Ok(())
}