use crate::value::Value;
use crate::name::*;
use crate::ast::*;
use crate::ast::Ast::*;
use std::collections::{HashMap, HashSet};
use crate::ir::IRStatement::ValueModify;

#[derive(Debug, Eq, PartialEq)]
pub struct IRFunction {
    params: Vec<Value>,
    body: IRBlock,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IRBlock {
    values: Vec<Value>,
    body: Vec<IRStatement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IRStatement {
    ValueModify(Value, i64),
    FunctionCall {
        func: IRFunction,
        args: Vec<Value>,
        target: Value,
    },
    Loop {
        condition_var: Value,
        body: Vec<IRStatement>,
    },
    Return(Value),
}

#[derive(Debug, Eq, PartialEq)]
pub struct IRProgram {
    inputs: Vec<Value>,
    output: Value,
    functions: HashMap<InternedName, IRFunction>,
    main: IRBlock,
}

struct Context {
    next_id: u64,
    context: HashMap<InternedName, Value>
}

impl Context {
    fn root() -> Self {
        Context {
            next_id: 0,
            context: HashMap::new()
        }
    }

    fn new_value(&mut self, name: InternedName) -> Value {
        let val = Value::new(self.next_id, name);
        self.next_id += 1;
        val
    }

    fn lookup_or_create(&mut self, name: &InternedName) -> Value {
        match self.context.get(name) {
            Some(&v) => v,
            None => {
                let v = self.new_value(*name);
                self.context.insert(*name, v);
                v
            }
        }
    }
}

fn convert_block(ctx: &mut Context, statements: &Vec<&Ast>) -> IRBlock {
    let mut ir = Vec::new();

    //TODO: Optimizations
    for &statement in statements {
        match statement {
            Incr(name) => {
                let v = ctx.lookup_or_create(name);
                ir.push(ValueModify(v, 1));
            }
            Decr(name) => {
                let v = ctx.lookup_or_create(name);
                ir.push(ValueModify(v, -1));
            }
            _ => panic!("Unexpected statement")
        }
    }
    unimplemented!()
}

fn convert_function(ctx: &mut Context, parameters: &Vec<InternedName>, body: &Vec<Ast>) -> IRFunction {
    unimplemented!()
}

pub fn convert_program_to_ir(program: &Program, name_store: &NameStore) -> Result<IRProgram, String> {
    let mut ctx = Context::root();
    let inputs: Vec<Value> = program.inputs.iter().map(|n| ctx.new_value(*n)).collect();
    let output = ctx.new_value(program.output);
    let mut functions = HashMap::new();
    let mut statements = Vec::new();

    for expr in &program.body {
        match expr {
            Def { name, parameters, body } => {
                functions.insert(*name, convert_function(&mut ctx, parameters, body));
            }
            _ => {
                statements.push(expr)
            }
        }
    }

    let block = convert_block(&mut ctx, &statements);

    let program = IRProgram {
        inputs,
        output,
        functions,
        main: block,
    };

    Ok(program)
}