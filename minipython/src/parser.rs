use crate::ast::*;
use crate::name::*;

lalrpop_mod!(pub minipython);

fn parse(code: &str) -> (NameStore, Result<Vec<Ast>, String>) {
    let mut name_store = NameStore::new();
    let parser = minipython::ProgramParser::new();
    let res = parser.parse(&mut name_store, code).map_err(|e| format!("{}", e));
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