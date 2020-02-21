use crate::ast::*;
use crate::name::*;
use crate::lexer::Lexer;

lalrpop_mod!(pub minipython);

fn parse_block(code: &str) -> (NameStore, Result<Vec<Ast>, String>) {
    let mut name_store = NameStore::new();
    let parser = minipython::BlockParser::new();
    let lexer = Lexer::new(code);
    let res = parser.parse(code, &mut name_store, lexer).map_err(|e| format!("{}", e));
    (name_store, res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ast::*;

    #[test]
    fn test_incr_decr() {
        let code = "a+=1";
        let (store, res) = parse_block(code);
        assert!(res.is_ok(), "{:#?}", res);
        let expected = vec![Incr {
            var_name: store.by_index(0).unwrap()
        }];
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_return() {
        let code = "return x";
        let (store, res) = parse_block(code);
        assert!(res.is_ok(), "{:#?}", res);
        let expected = vec![Return { name: store.by_index(0).unwrap() }];
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_while() {
        let code =
            "while x != 0:
    x-=1
";
        let (store, res) = parse_block(code);
        assert!(res.is_ok(), "{:#?}", res);
        let expected = vec![ While {
            cond_var: store.by_index(0).unwrap(),
            body: vec![
                Decr { var_name: store.by_index(0).unwrap() }
            ]
        } ];
        assert_eq!(res.unwrap(), expected);
    }
}