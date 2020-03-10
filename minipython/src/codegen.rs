use crate::ir::*;
use crate::name::*;
use std::error::Error;
use std::io::{BufWriter, Write};
use std::fs::File;
use crate::value::Value;
use crate::ir::IRStatement::{ValueModify, Return, Loop, FunctionCall};

const C_VALUE_TYPE: &str = "unsigned long long int";
const C_VALUE_FORMAT: &str = "%llu";

fn to_value_name(v: Value, name_store: &NameStore) -> String {
    format!("{}_{}", v.get_name(name_store).unwrap(), v.get_id())
}

fn write_value_init(output: &mut BufWriter<&File>, output_name: &String) -> Result<(), Box<dyn Error>> {
    writeln!(output, "{} {} = 0;", C_VALUE_TYPE, output_name)?;
    Ok(())
}

fn compile_statement(statement: &IRStatement, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    match statement {
        ValueModify(val, change) => {
            let val_name = to_value_name(*val, name_store);
            writeln!(output, "{} += {};", val_name, change)?;
        },
        Return(val) => {
            let val_name = to_value_name(*val, name_store);
            writeln!(output, "return {};", val_name)?;
        },
        Loop { condition_var, body } => {
            writeln!(output, "while ({}) {{", to_value_name(*condition_var, name_store))?;

            for st in body {
                compile_statement(st, name_store, output)?;
            }

            writeln!(output, "}}")?;
        },
        FunctionCall { func, args, target } => {
            let func_name = name_store.get(*func).unwrap();
            let target_name = to_value_name(*target, name_store);
            let args_names = args.iter().map(|&v| to_value_name(v, name_store)).collect::<Vec<String>>().join(", ");
            writeln!(output, "{} = {}({});", target_name, func_name, args_names)?;
        }
    }
    Ok(())
}

fn compile_block(block: &IRBlock, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    for &val in &block.values {
        let val_name = to_value_name(val, name_store);
        write_value_init(output, &val_name)?;
    }

    for statement in &block.body {
        compile_statement(statement, name_store, output)?;
    }

    Ok(())
}

pub fn compile_to_c(program: &IRProgram, name_store: &NameStore, output: &mut BufWriter<&File>) -> Result<(), Box<dyn Error>> {
    //Include stdio
    writeln!(output, "#include <stdio.h>")?;

    for (&function_name, function) in &program.functions {
        let params = function.params.iter().map(|&v| format!("{} {}", C_VALUE_TYPE, to_value_name(v, name_store))).collect::<Vec<String>>().join(", ");
        writeln!(output, "{} {}({}) {{", C_VALUE_TYPE, name_store.get(function_name).unwrap(), params)?;

        compile_block(&function.body, name_store, output)?;

        writeln!(output, "}}")?;
    }

    writeln!(output, "int main(int argc, char* argv[]) {{")?;

    for &input_val in &program.inputs {
        let val_name = to_value_name(input_val, name_store);
        writeln!(output, "{} {};", C_VALUE_TYPE, val_name)?;
        writeln!(output, "printf(\"{}=\");", input_val.get_name(name_store).unwrap())?;
        writeln!(output, "scanf(\"{}\", &{});", C_VALUE_FORMAT, val_name)?;
    }

    let output_name = to_value_name(program.output, name_store);
    write_value_init(output, &output_name)?;

    compile_block(&program.main, name_store, output)?;

    writeln!(output, "printf(\"{}={}\", {});", C_VALUE_FORMAT, program.output.get_name(name_store).unwrap(), output_name)?;
    writeln!(output, "return 0;")?;
    writeln!(output, "}}")?;

    Ok(())
}