use crate::ast::*;
use crate::name::*;
use crate::lexer::Lexer;

lalrpop_mod!(pub minipython);

fn parse_program(code: &str) -> (NameStore, Result<Program, String>) {
    let mut name_store = NameStore::new();
    let parser = minipython::ProgramParser::new();
    let lexer = Lexer::new(code);
    let res = parser.parse(code, &mut name_store, lexer).map_err(|e| format!("{}", e));
    (name_store, res)
}

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
        let expected = vec![While {
            cond_var: store.by_index(0).unwrap(),
            body: vec![
                Decr { var_name: store.by_index(0).unwrap() }
            ],
        }];
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_functions() {
        let code =
            "def add(a, b):
    while a != 0:
        a-=1
        b += 1
    return b
x+=1
y+=1
z=add(x, y)";
        let (store, res) = parse_block(code);
        assert!(res.is_ok(), "{:#?}", res);
        let add_var = store.get_by_interned("add").unwrap();
        let a_var = store.get_by_interned("a").unwrap();
        let b_var = store.get_by_interned("b").unwrap();
        let x_var = store.get_by_interned("x").unwrap();
        let y_var = store.get_by_interned("y").unwrap();
        let z_var = store.get_by_interned("z").unwrap();
        let expected = vec![Def {
            name: add_var,
            parameters: vec![a_var, b_var],
            body: vec![
                While {
                    cond_var: a_var,
                    body: vec![
                        Decr { var_name: a_var },
                        Incr { var_name: b_var }
                    ],
                },
                Return {
                    name: b_var
                }
            ],
        },
                            Incr { var_name: x_var },
                            Incr { var_name: y_var },
                            Assign {
                                var_name: z_var,
                                fun_name: add_var,
                                args: vec![x_var, y_var],
                            }];
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_program() {
        let code =
            "input: x, y
output: y
while x!=0:
    x-=1
    y+=1
";
        let (store, res) = parse_program(code);
        assert!(res.is_ok(), "{:#?}", res);
        let x_var = store.get_by_interned("x").unwrap();
        let y_var = store.get_by_interned("y").unwrap();
        let expected = Program {
            inputs: vec![x_var, y_var],
            output: y_var,
            body: vec! [ While {
                cond_var: x_var,
                body: vec! [Decr { var_name: x_var }, Incr { var_name: y_var }]
            }]
        };
        assert_eq!(res.unwrap(), expected);
    }
}