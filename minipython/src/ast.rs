use crate::name::InternedName;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ast {
    Def { name: InternedName, parameters: Vec<InternedName>, body: Vec<Ast> },
    Return(InternedName),
    While { cond_var: InternedName, body: Vec<Ast> },
    Assign { var_name: InternedName, fun_name: InternedName, args: Vec<InternedName> },
    Incr(InternedName),
    Decr(InternedName)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
    pub body: Vec<Ast>,
    pub inputs: Vec<InternedName>,
    pub output: InternedName
}