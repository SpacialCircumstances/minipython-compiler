use std::str::{CharIndices, Chars};
use crate::lexer::Token::*;
use std::collections::HashSet;
use std::iter::Peekable;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub enum NumLiteral {
    Zero,
    One
}

pub enum Token<'a>  {
    Input,
    Output,
    Comma,
    Ident,
    Unident,
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
            "!=" => NotEqual,
            "+=" => PlusEqual,
            "-=" => MinusEqual,
            _ => Name(lexeme)
        }
    }
}

pub enum LexerError {
    TabIdent
}

#[derive(Debug, Eq, PartialEq)]
pub struct Location {
    line: usize,
    col: usize,
    pos: usize
}


struct Position<'input> {
    chars: Chars<'input>,
    pos: usize,
    line: usize,
    col: usize
}

impl<'input> Position<'input> {
    fn from_chars(c: Chars<'input>) -> Position {
        Position {
            chars: c,
            pos: 0,
            line: 1,
            col: 1
        }
    }

    fn current(&self) -> Location {
        Location {
            pos: self.pos,
            line: self.line,
            col: self.col
        }
    }
}

impl<'input> Iterator for Position<'input> {
    type Item = (Location, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            None => None,
            Some('\n') => {
                let pos = self.current();
                self.line += 1;
                self.col = 1;
                self.pos += 1;
                Some((pos, '\n'))
            },
            Some(x) => {
                let pos = self.current();
                self.col += 1;
                self.pos += 1;
                Some((pos, x))
            }
        }
    }
}

pub enum LexerState {
    LineStart,
    Comment,
    Next
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
                            self.incr_pos();
                            break Some(Err(LexerError::TabIdent));
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

                                    let lexeme = &self.input[pos.pos..curr];
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