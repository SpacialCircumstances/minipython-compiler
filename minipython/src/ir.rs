use crate::value::Value;

pub struct IRFunction {
    params: Vec<Value>,
    body: Vec<IRStatement>
}

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

pub struct IRProgram {
    functions: Vec<IRFunction>,
    main: Vec<IRStatement>
}