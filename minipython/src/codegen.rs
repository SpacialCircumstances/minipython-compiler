use std::error::Error;
use crate::ir::*;
use crate::name::NameStore;

pub fn generate_llvm_code(program: &IRProgram, name_store: &NameStore) -> Result<(), Box<dyn Error>> {
    Ok(())
}