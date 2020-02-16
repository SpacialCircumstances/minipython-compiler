use crate::name::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Def { name: Name, parameters: Vec<Name>, body: Vec<Ast> },
    Return { name: Name },
    While { cond_var: Name, body: Vec<Ast> },
    Assign { var_name: Name, fun_name: Name, args: Vec<Name> },
    Incr { var_name: Name },
    Decr { var_name: Name }
}