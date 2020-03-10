use std::path::Path;
use std::fs;
use crate::parser;
use crate::ir;
use crate::codegen;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CompilerInstance<'a> {
    input_file: &'a Path,
    output_file: &'a Path,
    compile: bool
}

impl<'a> CompilerInstance<'a> {
    pub fn new(input_file: &'a Path, output_file: &'a Path, compile: bool) -> Result<CompilerInstance<'a>, String> {
        if input_file.exists() {
            Ok(CompilerInstance {
                input_file,
                output_file,
                compile
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
        let file = File::create(self.output_file).map_err(|e| format!("{}", e))?;
        let mut writer = BufWriter::new(&file);
        codegen::compile_to_c(&ir, &name_store, &mut writer).map_err(|e| format!("{}", e))?;
        writer.flush().map_err(|e| format!("{}", e))?;
        Ok(())
    }
}