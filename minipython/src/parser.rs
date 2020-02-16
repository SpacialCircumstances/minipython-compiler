use crate::ast::*;
use crate::name::*;

lalrpop_mod!(pub minipython);

fn parse(code: &str) -> Ast {
    let mut name_store = NameStore::new();
    let parser = minipython::ExprParser::new();
    parser.parse(&mut name_store, code).unwrap()
}