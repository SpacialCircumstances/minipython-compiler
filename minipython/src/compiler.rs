use std::path::Path;
use std::fs;
use crate::parser;
use crate::ir;

pub struct CompilerInstance<'a> {
    input_file: &'a Path,
    output_file: &'a Path
}

impl<'a> CompilerInstance<'a> {
    pub fn new(input_file: &'a Path, output_file: &'a Path) -> Result<CompilerInstance<'a>, String> {
        if input_file.exists() {
            Ok(CompilerInstance {
                input_file,
                output_file
            })
        } else {
            Err(String::from(format!("Input file {} does not exist", input_file.display())))
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let code = fs::read_to_string(self.input_file).map_err(|e| format!("{}", e))?;
        let (name_store, ast_res) = parser::parse_program(&code);
        let ast = ast_res?;
        let ir = ir::convert_program_to_ir(&ast, &name_store)?;
        Ok(())
    }
}