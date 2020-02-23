use std::path::Path;

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
}