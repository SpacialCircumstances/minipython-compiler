#[derive(Debug, Eq, PartialEq)]
pub enum Ast<Name> {
    Def { name: Name, parameters: Vec<Name>, body: Vec<Ast<Name>> },
    Return { name: Name },
    While { cond_var: Name, body: Vec<Ast<Name>> },
    Assign { var_name: Name, fun_name: Name, args: Vec<Name> },
    Incr { var_name: Name },
    Decr { var_name: Name }
}