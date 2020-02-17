use std::str::{CharIndices, Chars};
use crate::lexer::Token::*;
use std::collections::HashSet;
use std::iter::Peekable;
use crate::accumulator::{Accumulator, Accumulateable, Acc};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum NumLiteral {
    Zero,
    One
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token<'a>  {
    Input,
    Output,
    Comma,
    Indent,
    Unindent,
    Return,
    Def,
    Colon,
    Name(&'a str),
    OpenParen,
    CloseParen,
    While,
    NotEqual,
    Literal(NumLiteral),
    PlusEqual,
    MinusEqual
}

impl<'a> Token<'a> {
    fn from_lexeme(lexeme: &'a str) -> Token {
        match lexeme {
            "while" => While,
            "input" => Input,
            "output" => Output,
            "def" => Def,
            "return" => Return,
            "!=" => NotEqual,
            "+=" => PlusEqual,
            "-=" => MinusEqual,
            _ => Name(lexeme)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Location {
    line: usize,
    col: usize,
    pos: usize
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LexerErrorKind {
    TabIdent
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LexerError {
    position: Location,
    kind: LexerErrorKind
}

impl LexerError {
    fn new(position: Location, kind: LexerErrorKind) -> Self {
        LexerError {
            kind,
            position
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CurrentToken {
    Indent,
    NextToken,
    Comment,
    Name(Location)
}

fn is_separator(c: char) -> bool {
    match c {
        ':' | ',' | ' ' | '\n' | '\t' | '\r' | '(' | ')' | '!' | '+' | '-' | '#' => true,
        _ => false
    }
}

fn single_char_token<'a>(c: char) -> Option<Token<'a>> {
    match c {
        ':' => Some(Colon),
        ',' => Some(Comma),
        '(' => Some(OpenParen),
        ')' => Some(CloseParen),
        '0' => Some(Literal(NumLiteral::Zero)),
        '1' => Some(Literal(NumLiteral::One)),
        _ => None
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct LexerState<'input> {
    pub pos: Location,
    pub current_token: CurrentToken,
    pub last_indent_level: usize,
    pub indent_level: usize,
    pub input: &'input str
}

impl<'input> LexerState<'input> {
    fn new(input: &'input str) -> Self {
        LexerState {
            pos: Location {
                pos: 0,
                line: 1,
                col: 1
            },
            current_token: CurrentToken::Indent,
            last_indent_level: 0,
            indent_level: 0,
            input
        }
    }

    fn incr_line(&self) -> Self {
        LexerState {
            pos: Location {
                line: self.pos.line + 1,
                pos: self.pos.pos + 1,
                col: 1
            },
            current_token: self.current_token,
            last_indent_level: self.indent_level,
            input: self.input,
            indent_level: 0
        }
    }

    fn incr_pos(&self) -> Self {
        LexerState {
            pos: Location {
                pos: self.pos.pos + 1,
                line: self.pos.line,
                col: self.pos.col
            },
            current_token: self.current_token,
            indent_level: self.indent_level,
            last_indent_level: self.last_indent_level,
            input: self.input
        }
    }

    fn current_by_token_start(&mut self, c: char) {
        self.current_token = match c {
            '#' => CurrentToken::Comment,
            '\n' => CurrentToken::Indent,
            ' ' | '\t' => CurrentToken::NextToken,
            _ => CurrentToken::Name(self.pos)
        }
    }
}

type LexerResult<'input> = Spanned<Token<'input>, Location, LexerError>;

fn lex_step<'input>(it: Option<char>, state: &LexerState<'input>) -> (LexerState<'input>, Acc<LexerResult<'input>>) {
    match it {
        None => (state.clone(), Acc::End), //TODO: Check if string literal etc is finished
        Some(c) => match state.current_token {
                CurrentToken::Indent => {
                    match c {
                        ' ' | '\t' => {
                            let mut next_state = state.incr_pos();
                            next_state.indent_level += 1;
                            (next_state, Acc::Continue)
                        }
                        _ => {
                            let mut next_state = if c == '\n' {
                                state.incr_line()
                            } else {
                                state.incr_pos()
                            };
                            next_state.current_by_token_start(c);
                            if state.indent_level < state.last_indent_level {
                                (next_state, Acc::Next(Ok((state.pos, Token::Unindent, next_state.pos))))
                            } else if state.indent_level > state.last_indent_level {
                                (next_state, Acc::Next(Ok((state.pos, Token::Indent, next_state.pos))))
                            } else {
                                (next_state, Acc::Continue)
                            }
                        }
                    }
                },
                CurrentToken::NextToken => {
                    let mut next_state = if c == '\n' {
                        state.incr_line()
                    } else {
                        state.incr_pos()
                    };
                    next_state.current_by_token_start(c);
                    (next_state, Acc::Continue)
                },
                CurrentToken::Comment => {
                    if c == '\n' {
                        let mut next_state = state.incr_line();
                        next_state.current_token = CurrentToken::Indent;
                        (next_state, Acc::Continue)
                    } else {
                        (state.incr_pos(), Acc::Continue)
                    }
                }
                CurrentToken::Name(start) => {
                    if is_separator(c) {
                        let end = state.pos.pos;
                        let lexeme = &state.input[start.pos..end];
                        let mut next_state = if c == '\n' {
                            state.incr_line()
                        } else {
                            state.incr_pos()
                        };
                        next_state.current_by_token_start(c);
                        let token = if lexeme.len() == 1 {
                            single_char_token(lexeme.chars().nth(0).unwrap()).unwrap_or(Token::from_lexeme(lexeme))
                        } else {
                            Token::from_lexeme(lexeme)
                        };
                        (next_state, Acc::Next(Ok((start, token, state.pos))))
                    } else {
                        (state.incr_pos(), Acc::Continue)
                    }
                }
            }
    }
}

fn lex<'input>(input: &'input str) -> Accumulator<Chars<'input>, fn(Option<char>, &LexerState<'input>) -> (LexerState<'input>, Acc<LexerResult<'input>>), LexerState, LexerResult<'input>> {
    let mut initial_state = LexerState::new(input);
    input.chars().accumulate(lex_step, initial_state)
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, NumLiteral, Spanned, Location, LexerError, lex};
    use crate::lexer::Token::*;
    use crate::lexer::NumLiteral::*;

    fn lex_equal(code: &str, tokens: Vec<Token>) {
        let res: Vec<Token> = lex(code).map(|tk| tk.unwrap()).map(|(_, t, _)| t).collect();
        assert_eq!(res, tokens);
    }

    #[test]
    fn test_lexer_1() {
        let code = "while a != 0:";
        let tokens = vec![
            While,
            Name("a"),
            NotEqual,
            Literal(Zero),
            Colon
        ];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_2() {
        let code = "return x";
        let tokens = vec![ Token::Return, Name("x") ];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_3() {
        let code = "a += 1 #test";
        let tokens = vec![ Name("a"), PlusEqual, Literal(One) ];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_4() {
        let code = "input: a, b, c";
        let tokens = vec![ Input, Colon, Name("a"), Comma, Name("b"), Comma, Name("c") ];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_5() {
        let code = "def a(b, c, d): a += 1";
        let tokens = vec![ Def, Name("a"), OpenParen, Name("b"), Comma, Name("c"), Comma, Name("d"), CloseParen, Colon, Name("a"), PlusEqual, Literal(One) ];
        lex_equal(code, tokens);
    }
}