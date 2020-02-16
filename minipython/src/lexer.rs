use std::str::{CharIndices, Chars};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub enum NumLiteral {
    Zero,
    One
}

pub enum Token  {
    Input,
    Output,
    Comma,
    Ident,
    Unident,
    Def,
    Colon,
    Name,
    OpenParen,
    CloseParen,
    While,
    NotEqual,
    Literal(NumLiteral),
    PlusEqual,
    MinusEqual
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
    chars: Chars<'input>,
    identation_level: i32,
    line: usize,
    pos: usize,
    col: usize
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.chars(),
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

    fn single_char_token(c: char) -> Option<Token> {
        None
    }

    fn comment(&mut self) {

    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, Location, LexerError>;

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
                            self.incr_pos();
                        }
                        '\t' => {
                            break Some(Err(LexerError::TabIdent));
                        }
                        '\r' => self.incr_pos(),
                        '#' => self.comment(),
                        _ => {
                            let pos = self.current_pos();
                            self.incr_pos();
                            match Lexer::single_char_token(c) {
                                Some(tk) => {
                                    break Some(Ok((pos, tk, self.current_pos())));
                                },
                                None => {
                                    //TODO
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}