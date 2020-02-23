use crate::value::Value;
use crate::name::*;
use crate::ast::*;
use crate::ast::Ast::*;

#[derive(Debug, Eq, PartialEq)]
pub struct IRFunction {
    params: Vec<Value>,
    body: Vec<IRStatement>
}

#[derive(Debug, Eq, PartialEq)]
pub enum IRStatement {
    ValueCreate(Value),
    ValueModify(Value, i64),
    FunctionCall {
        func: IRFunction,
        args: Vec<Value>,
        target: Value
    },
    Loop {
        condition_var: Value,
        body: Vec<IRStatement>
    },
    Return(Value)
}

#[derive(Debug, Eq, PartialEq)]
pub struct IRProgram {
    inputs: Vec<Value>,
    output: Value,
    functions: Vec<IRFunction>,
    main: Vec<IRStatement>
}

pub fn convert_program_to_ir(program: &Program, name_store: &NameStore) -> Result<IRProgram, String> {
    Err(String::from("Not implemented"))
}