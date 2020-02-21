use crate::name::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Def { name: Name, parameters: Vec<Name>, body: Vec<Ast> },
    Return(Name),
    While { cond_var: Name, body: Vec<Ast> },
    Assign { var_name: Name, fun_name: Name, args: Vec<Name> },
    Incr(Name),
    Decr(Name)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub body: Vec<Ast>,
    pub inputs: Vec<Name>,
    pub output: Name
}