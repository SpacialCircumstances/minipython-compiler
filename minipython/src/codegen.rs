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

pub fn compile_block(block: &IRBlock, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    for &val in &block.values {
        let val_name = to_value_name(val, name_store);
        writeln!(output, "{} {};", C_VALUE_TYPE, val_name)?;
    }

    Ok(())
}

pub fn compile_to_c(program: &IRProgram, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    //Include stdio
    writeln!(output, "#include <stdio.h>")?;

    for (&function_name, function) in &program.functions {
        let params = function.params.iter().map(|&v| to_value_name(v, name_store)).collect::<Vec<String>>().join(", ");
        writeln!(output, "int {}({}) {{", name_store.get(function_name).unwrap(), params)?;

        compile_block(&function.body, name_store, output)?;

        writeln!(output, "}}")?;
    }

    writeln!(output, "int main(int argc, char* argv[]) {{")?;

    for &input_val in &program.inputs {
        let val_name = to_value_name(input_val, name_store);
        writeln!(output, "{} {};", C_VALUE_TYPE, val_name)?;
        writeln!(output, "printf(\"{}=\");", input_val.get_name(name_store).unwrap())?;
        writeln!(output, "scanf(\"%llu\", &{});", val_name)?;
    }

    let output_name = to_value_name(program.output, name_store);
    writeln!(output, "{} {};", C_VALUE_TYPE, output_name)?;

    compile_block(&program.main, name_store, output)?;

    writeln!(output, "return 0;")?;
    writeln!(output, "}}")?;

    Ok(())
}