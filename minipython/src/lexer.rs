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
    NextToken
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct LexerState {
    pub pos: Location,
    pub current_token: CurrentToken
}

impl LexerState {
    fn new() -> Self {
        LexerState {
            pos: Location {
                pos: 0,
                line: 1,
                col: 1
            },
            current_token: CurrentToken::Indent
        }
    }

    fn incr_line(&self) -> Self {
        LexerState {
            pos: Location {
                line: self.pos.line + 1,
                pos: self.pos.pos + 1,
                col: 1
            },
            current_token: self.current_token
        }
    }

    fn incr_pos(&self) -> Self {
        LexerState {
            pos: Location {
                pos: self.pos.pos + 1,
                line: self.pos.line,
                col: self.pos.col
            },
            current_token: self.current_token
        }
    }
}

type LexerResult<'input> = Spanned<Token<'input>, Location, LexerError>;

fn lex_step<'input>(it: Option<char>, state: &LexerState) -> (LexerState, Acc<LexerResult<'input>>) {
    match it {
        None => (state.clone(), Acc::End),
        Some(c) => match state.current_token {
                CurrentToken::Indent => {
                    (LexerState::new(), Acc::Continue)
                },
                CurrentToken::NextToken => {
                    (LexerState::new(), Acc::Continue)
                }
            }
    }
}

fn lex<'input>(input: &'input str) -> Accumulator<Chars<'input>, fn(Option<char>, &LexerState) -> (LexerState, Acc<LexerResult<'input>>), LexerState, LexerResult<'input>> {
    let mut initial_state = LexerState::new();
    input.chars().accumulate(lex_step, initial_state)
}

pub struct Lexer<'input> {
    chars: Peekable<Chars<'input>>,
    input: &'input str,
    identation_level: i32,
    line: usize,
    pos: usize,
    col: usize
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            input,
            identation_level: 0,
            line: 1,
            pos: 0,
            col: 1
        }
    }

    fn current_pos(&self) -> Location {
        Location {
            pos: self.pos,
            line: self.line,
            col: self.col
        }
    }

    fn incr_line(&mut self) {
        self.line += 1;
        self.col = 1;
        self.pos += 1;
    }

    fn incr_pos(&mut self) {
        self.col += 1;
        self.pos += 1;
    }

    fn comment(&mut self) {
        loop {
            match self.chars.next() {
                None => break,
                Some('\n') => {
                    self.incr_line();
                    break;
                }
                Some(_) => self.incr_pos()
            }
        }
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

fn is_separator(c: char) -> bool {
    match c {
        ':' | ',' | ' ' | '\n' | '\t' | '\r' | '(' | ')' | '!' | '+' | '-' | '#' => true,
        _ => false
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, Location, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                None => break None,
                Some(c) => {
                    match c {
                        '\n' => {
                            self.incr_line();
                        }
                        ' ' => {
                            //TODO: Identation
                            self.incr_pos();
                        }
                        '\t' => {
                            let pos = self.current_pos();
                            self.incr_pos();
                            break Some(Err(LexerError::new(pos, LexerErrorKind::TabIdent)));
                        }
                        '\r' => self.incr_pos(),
                        '#' => {
                            self.incr_pos();
                            self.comment()
                        },
                        _ => {
                            let pos = self.current_pos();
                            self.incr_pos();
                            match single_char_token(c) {
                                Some(tk) => {
                                    break Some(Ok((pos, tk, self.current_pos())));
                                },
                                None => {
                                    let mut curr = pos.pos;
                                    while let Some(next) = self.chars.peek() {
                                        if is_separator(*next) {
                                            break;
                                        } else {
                                            self.incr_pos();
                                            self.chars.next();
                                            curr += 1;
                                        }
                                    }

                                    let lexeme = &self.input[pos.pos..=curr];
                                    let token = Token::from_lexeme(lexeme);
                                    break (Some(Ok((pos, token, self.current_pos()))));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, NumLiteral, Lexer, Spanned, Location, LexerError};
    use crate::lexer::Token::*;
    use crate::lexer::NumLiteral::*;

    fn lex_equal(code: &str, tokens: Vec<Token>) {
        let lexer = Lexer::new(code);
        let res: Vec<Token> = lexer.map(|tk| tk.unwrap()).map(|(_, t, _)| t).collect();
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