use crate::value::Value;
use crate::name::*;
use crate::ast::*;
use crate::ast::Ast::*;
use std::collections::{HashMap, HashSet};
use crate::ir::IRStatement::{ValueModify, FunctionCall};
use std::rc::Rc;
use std::ops::Deref;
use std::borrow::BorrowMut;

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
        func: InternedName,
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
    next_id: Rc<u64>,
    context: HashMap<InternedName, Value>
}

impl Context {
    fn root() -> Self {
        Context {
            next_id: Rc::new(0),
            context: HashMap::new()
        }
    }

    fn create_subcontext(&mut self) -> Self {
        Context {
            next_id: self.next_id.clone(),
            context: HashMap::new()
        }
    }

    fn new_value(&mut self, name: InternedName) -> Value {
        let counter = Rc::get_mut(&mut self.next_id).unwrap();
        let val = Value::new(*counter, name);
        *counter += 1;
        self.context.insert(name, val);
        val
    }

    fn lookup_or_create(&mut self, name: &InternedName) -> Value {
        match self.context.get(name) {
            Some(&v) => v,
            None => {
                self.new_value(*name)
            }
        }
    }

    fn get_context_values(&self) -> Vec<Value> {
        self.context.iter().map(|(_, &v)| v).collect()
    }
}

struct OptimizationContext {
    values: HashMap<Value, i64>
}

impl OptimizationContext {
    fn new() -> Self {
        OptimizationContext {
            values: HashMap::new()
        }
    }

    fn incr(&mut self, v: Value) {
        let old = self.values.get_mut(&v).unwrap();
        *old += 1;
    }

    fn decr(&mut self, v: Value) {
        let old = self.values.get_mut(&v).unwrap();
        *old -= 1;
    }

    fn flush(&mut self, target: &mut Vec<IRStatement>) {
        for (&val, &modification) in &self.values {
            target.push(ValueModify(val, modification));
        }
        self.values.clear();
    }
}

fn convert_statements(ctx: &mut Context, statements: &Vec<Ast>) -> Vec<IRStatement> {
    let mut ir = Vec::new();
    let mut opt = OptimizationContext::new();

    for statement in statements {
        match statement {
            Incr(name) => {
                let v = ctx.lookup_or_create(name);
                opt.incr(v);
            }
            Decr(name) => {
                let v = ctx.lookup_or_create(name);
                opt.decr(v);
            }
            Return(name) => {
                opt.flush(&mut ir);
                let v = ctx.lookup_or_create(name);
                ir.push(IRStatement::Return(v));
            }
            Assign { var_name, fun_name, args } => {
                //We only *need* to flush variables used in the statement
                opt.flush(&mut ir);
                let args_values = args.iter().map(|n| ctx.lookup_or_create(n)).collect();
                let target = ctx.lookup_or_create(var_name);
                ir.push(FunctionCall {
                    func: *fun_name,
                    args: args_values,
                    target
                });
            }
            _ => panic!("Unexpected statement")
        }
    }

    ir
}

fn convert_block(ctx: &mut Context, statements: &Vec<Ast>) -> IRBlock {
    let ir_statements = convert_statements(ctx, statements);
    IRBlock {
        values: ctx.get_context_values(),
        body: ir_statements
    }
}

fn convert_function(ctx: &mut Context, parameters: &Vec<InternedName>, body: &Vec<Ast>) -> IRFunction {
    let mut func_ctx = ctx.create_subcontext();
    IRFunction {
        params: parameters.iter().map(|&n| func_ctx.new_value(n)).collect(),
        body: convert_block(&mut func_ctx, body)
    }
}

pub fn convert_program_to_ir(program: &Program, name_store: &NameStore) -> Result<IRProgram, String> {
    let mut ctx = Context::root();
    let inputs: Vec<Value> = program.inputs.iter().map(|n| ctx.new_value(*n)).collect();
    let output = ctx.new_value(program.output);
    let mut functions = HashMap::new();
    let mut statements: Vec<Ast> = Vec::new();

    for expr in &program.body {
        match expr {
            Def { name, parameters, body } => {
                functions.insert(*name, convert_function(&mut ctx, parameters, body));
            }
            _ => {
                statements.push(expr.clone())
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