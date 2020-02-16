use crate::ast::*;
use crate::name::*;

lalrpop_mod!(pub minipython);

fn parse(code: &str) -> Result<Vec<Ast>, String> {
    let mut name_store = NameStore::new();
    let parser = minipython::ProgramParser::new();
    parser.parse(&mut name_store, code).map_err(|e| format!("{}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr_decr() {
        let code = "a+=1";
        let res = parse(code);
        assert!(res.is_ok())
    }
}