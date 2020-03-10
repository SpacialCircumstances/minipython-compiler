use crate::value::Value;
use crate::name::*;
use crate::ast::*;
use crate::ast::Ast::*;
use std::collections::{HashMap, BTreeMap};
use crate::ir::IRStatement::{ValueModify, FunctionCall, Loop};
use std::rc::Rc;
use std::ops::Deref;
use std::borrow::BorrowMut;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
pub struct IRFunction {
    pub params: Vec<Value>,
    pub body: IRBlock,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IRBlock {
    pub values: Vec<Value>,
    pub body: Vec<IRStatement>,
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
    pub inputs: Vec<Value>,
    pub output: Value,
    pub functions: HashMap<InternedName, IRFunction>,
    pub main: IRBlock,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum ValueKind {
    IO,
    Normal,
}

struct Context {
    next_id: Rc<RefCell<u64>>,
    context: BTreeMap<InternedName, (ValueKind, Value)>,
    function_calls: Vec<Ast>,
}

impl Context {
    fn root() -> Self {
        Context {
            next_id: Rc::new(RefCell::new(0)),
            context: BTreeMap::new(),
            function_calls: Vec::new()
        }
    }

    fn create_subcontext(&mut self) -> Self {
        Context {
            next_id: self.next_id.clone(),
            context: BTreeMap::new(),
            function_calls: Vec::new()
        }
    }

    fn new_value(&mut self, name: InternedName) -> Value {
        let old = *self.next_id.borrow().deref();
        let val = Value::new(old, name);
        self.next_id.borrow_mut().replace(old + 1);
        self.context.insert(name, (ValueKind::Normal, val));
        val
    }

    fn new_io_value(&mut self, name: InternedName) -> Value {
        let old = *self.next_id.borrow().deref();
        let val = Value::new(old, name);
        self.next_id.borrow_mut().replace(old + 1);
        self.context.insert(name, (ValueKind::IO, val));
        val
    }

    fn lookup_or_create(&mut self, name: &InternedName) -> Value {
        match self.context.get(name) {
            Some((_, v)) => *v,
            None => {
                self.new_value(*name)
            }
        }
    }

    fn get_context_values(&self) -> Vec<Value> {
        self.context.iter().filter_map(|(_, (vk, v))| match vk {
            ValueKind::Normal => Some(*v),
            ValueKind::IO => None
        }).collect()
    }
}

struct OptimizationContext {
    values: BTreeMap<Value, i64>
}

impl OptimizationContext {
    fn new() -> Self {
        OptimizationContext {
            values: BTreeMap::new()
        }
    }

    fn incr(&mut self, v: Value) {
        let old = self.values.entry(v).or_insert(0);
        *old += 1;
    }

    fn decr(&mut self, v: Value) {
        let old = self.values.entry(v).or_insert(0);
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
                //Record function call for checking it later
                ctx.function_calls.push(statement.clone());
                //We only *need* to flush variables used in the statement
                opt.flush(&mut ir);
                let args_values = args.iter().map(|n| ctx.lookup_or_create(n)).collect();
                let target = ctx.lookup_or_create(var_name);
                ir.push(FunctionCall {
                    func: *fun_name,
                    args: args_values,
                    target,
                });
            }
            While { cond_var, body } => {
                opt.flush(&mut ir);
                let cond_val = ctx.lookup_or_create(cond_var);
                let ir_body = convert_statements(ctx, body);
                ir.push(Loop {
                    condition_var: cond_val,
                    body: ir_body,
                });
            }
            _ => panic!("Unexpected statement")
        }
    }

    opt.flush(&mut ir);
    ir
}

fn convert_block(ctx: &mut Context, statements: &Vec<Ast>, check_return: bool) -> Result<IRBlock, String> {
    let ir_statements = convert_statements(ctx, statements);
    let has_return = !check_return || ir_statements.iter().any(|st| match st {
        IRStatement::Return(_) => true,
        _ => false
    });
    if has_return {
        Ok(IRBlock {
            values: ctx.get_context_values(),
            body: ir_statements,
        })
    } else {
        Err(String::from("Block has no return value"))
    }
}

fn convert_function(ctx: &mut Context, parameters: &Vec<InternedName>, body: &Vec<Ast>) -> Result<IRFunction, String> {
    let mut func_ctx = ctx.create_subcontext();
    let func = IRFunction {
        params: parameters.iter().map(|&n| func_ctx.new_io_value(n)).collect(),
        body: convert_block(&mut func_ctx, body, true)?,
    };
    ctx.function_calls.append(&mut func_ctx.function_calls);
    Ok(func)
}

fn convert_program(ctx: &mut Context, program: &Program) -> Result<IRProgram, String> {
    let inputs: Vec<Value> = program.inputs.iter().map(|n| ctx.new_io_value(*n)).collect();
    let output = ctx.new_io_value(program.output);
    let mut functions = HashMap::new();
    let mut statements: Vec<Ast> = Vec::new();

    for expr in &program.body {
        match expr {
            Def { name, parameters, body } => {
                functions.insert(*name, convert_function(ctx, parameters, body)?);
            }
            _ => {
                statements.push(expr.clone())
            }
        }
    }

    let block = convert_block(ctx, &statements, false)?;

    Ok(IRProgram {
        inputs,
        output,
        functions,
        main: block,
    })
}

pub fn convert_program_to_ir(program: &Program, name_store: &NameStore) -> Result<IRProgram, String> {
    let mut ctx = Context::root();
    let ir_prog = convert_program(&mut ctx, program)?;

    let func_calls = ctx.function_calls;

    for x in func_calls {
        match x {
            Assign { var_name, fun_name, args } => {
                let func = ir_prog.functions.get(&fun_name);
                let f = name_store.get(fun_name).unwrap();
                let v = name_store.get(var_name).unwrap();
                match func {
                    Some(func) => {
                        if func.params.len() != args.len() {
                            return Err(format!("Error assigning to variable {}: Function {} requires {} arguments, but got {}", v, f, func.params.len(), args.len()))
                        }
                    },
                    None => {
                        return Err(format!("Error assigning to variable {}: Function {} does not exist", v, f))
                    }
                }
            },
            _ => unreachable!()
        }
    }

    Ok(ir_prog)
}

#[cfg(test)]
mod tests {
    use crate::ast::Program;
    use crate::name::NameStore;
    use crate::ast::Ast::{While, Decr, Incr, Def, Return, Assign};
    use crate::ir::{convert_program_to_ir, IRProgram, IRBlock, IRFunction, IRStatement, Context, convert_program};
    use crate::value::Value;
    use std::collections::HashMap;
    use crate::ir::IRStatement::{ValueModify, Loop, FunctionCall};
    use crate::parser::parse_program;

    #[test]
    fn test_function_call_collection() {
        let code =
            "input:
output: f
def add(a, b):
    while a!=0:
        a-=1
        b+=1
    return b

def mul(a, b):
    while a!=0:
        a-=1
        c=add(b, c)
    return c

d+=1
e+=1
f=mul(d, e)";

        let (store, ast_res) = parse_program(code);
        let ast = ast_res.unwrap();
        let mut ctx = Context::root();
        let _ir_prog = convert_program(&mut ctx, &ast);
        let func_calls = ctx.function_calls;
        let c_v = store.by_index(5).unwrap();
        let expected = vec![
            Assign {
                var_name: c_v,
                fun_name: store.by_index(1).unwrap(),
                args: vec![
                    store.by_index(3).unwrap(),
                    c_v
                ]
            },
            Assign {
                var_name: store.by_index(0).unwrap(),
                fun_name: store.by_index(4).unwrap(),
                args: vec![
                    store.by_index(6).unwrap(),
                    store.by_index(7).unwrap()
                ]
            }
        ];
        assert_eq!(func_calls, expected);
        assert!(convert_program_to_ir(&ast, &store).is_ok());
    }

    #[test]
    fn test_program_conversion() {
        let mut name_store = NameStore::new();
        let a_var = name_store.register("a");
        let b_var = name_store.register("b");
        let c_var = name_store.register("c");
        let incr_2_var = name_store.register("incr_2");
        let ret_var = name_store.register("ret");
        let program = Program {
            inputs: vec![a_var],
            output: ret_var,
            body: vec![
                Def {
                    name: incr_2_var,
                    parameters: vec![a_var],
                    body: vec![
                        Incr(a_var),
                        Incr(a_var),
                        Incr(b_var),
                        Return(a_var)
                    ],
                },
                Incr(b_var),
                Incr(b_var),
                Incr(ret_var),
                Assign {
                    var_name: c_var,
                    fun_name: incr_2_var,
                    args: vec![b_var],
                }
            ],
        };

        let converted = convert_program_to_ir(&program, &name_store);
        assert!(converted.is_ok());
        let a_val = Value::new(0, a_var);
        let ret_val = Value::new(1, ret_var);
        let a_val2 = Value::new(2, a_var);
        let b_val2 = Value::new(3, b_var);
        let b_val = Value::new(4, b_var);
        let c_val = Value::new(5, c_var);

        let mut expected_functions = HashMap::new();
        expected_functions.insert(incr_2_var, IRFunction {
            params: vec![a_val2],
            body: IRBlock {
                values: vec![b_val2],
                body: vec![
                    ValueModify(a_val2, 2),
                    ValueModify(b_val2, 1),
                    IRStatement::Return(a_val2)
                ],
            },
        });

        let expected = IRProgram {
            inputs: vec![a_val],
            output: ret_val,
            main: IRBlock {
                values: vec![b_val, c_val],
                body: vec![
                    ValueModify(ret_val, 1),
                    ValueModify(b_val, 2),
                    FunctionCall {
                        func: incr_2_var,
                        target: c_val,
                        args: vec![b_val],
                    }
                ],
            },
            functions: expected_functions,
        };

        assert_eq!(converted.unwrap(), expected);
    }

    #[test]
    fn test_io_conversion() {
        let mut name_store = NameStore::new();
        let a_var = name_store.register("a");
        let b_var = name_store.register("b");
        let c_var = name_store.register("c");
        let d_var = name_store.register("d");
        let ret_var = name_store.register("ret");
        let program = Program {
            inputs: vec![
                a_var,
                b_var,
                c_var
            ],
            output: ret_var,
            body: vec![
                Incr(d_var),
                While {
                    cond_var: a_var,
                    body: vec![
                        Decr(a_var),
                        Incr(ret_var)
                    ],
                }
            ],
        };

        let converted = convert_program_to_ir(&program, &name_store);
        assert!(converted.is_ok());
        let a_val = Value::new(0, a_var);
        let b_val = Value::new(1, b_var);
        let c_val = Value::new(2, c_var);
        let ret_val = Value::new(3, ret_var);
        let d_val = Value::new(4, d_var);
        let expected = IRProgram {
            inputs: vec![a_val, b_val, c_val],
            output: ret_val,
            functions: HashMap::new(),
            main: IRBlock {
                values: vec![d_val],
                body: vec![
                    ValueModify(d_val, 1),
                    Loop {
                        condition_var: a_val,
                        body: vec![
                            ValueModify(a_val, -1),
                            ValueModify(ret_val, 1)
                        ],
                    }
                ],
            },
        };
        assert_eq!(converted.unwrap(), expected);
    }
}