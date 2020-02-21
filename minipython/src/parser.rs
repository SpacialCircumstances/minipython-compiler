use crate::ast::*;
use crate::name::*;
use crate::lexer::Lexer;

lalrpop_mod!(pub minipython);

fn parse(code: &str) -> (NameStore, Result<Vec<Ast>, String>) {
    let mut name_store = NameStore::new();
    let parser = minipython::ProgramParser::new();
    let lexer = Lexer::new(code);
    let res = parser.parse(code, &mut name_store, lexer).map_err(|e| format!("{}", e));
    (name_store, res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ast::Incr;

    #[test]
    fn test_incr_decr() {
        let code = "a+=1";
        let (store, res) = parse(code);
        let expected = vec![Incr {
            var_name: store.by_index(0).unwrap()
        }];
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }
}