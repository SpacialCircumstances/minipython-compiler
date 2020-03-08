use crate::ir::*;
use crate::name::*;
use std::error::Error;
use std::io::{BufWriter, Write};
use std::fs::File;
use crate::value::Value;

const C_VALUE_TYPE: &str = "unsigned long long";

fn to_value_name(v: Value, name_store: &NameStore) -> String {
    format!("{}_{}", v.get_name(name_store).unwrap(), v.get_id())
}

pub fn compile_to_c(program: &IRProgram, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    //Include stdio
    writeln!(output, "#include <stdio.h>")?;

    //TODO: Functions

    writeln!(output, "int main(int argc, char* argv[]) {{")?;

    for &input_val in &program.inputs {
        let val_name = to_value_name(input_val, name_store);
        writeln!(output, "{} {};", C_VALUE_TYPE, val_name)?;
        writeln!(output, "printf(\"{}=\");", input_val.get_name(name_store).unwrap())?;
        writeln!(output, "scanf(\"%llu\", &{});", val_name)?;
    }

    writeln!(output, "{} {};", C_VALUE_TYPE, to_value_name(program.output, name_store))?;

    //TODO: Main block

    writeln!(output, "return 0;")?;
    writeln!(output, "}}")?;

    Ok(())
}