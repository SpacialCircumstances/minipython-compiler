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

pub struct Location {
    line: usize,
    col: usize,
    pos: usize
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
    col: usize,
    state: LexerState
}

struct Position<'input> {
    chars: Chars<'input>,
    pos: usize,
    line: usize,
    col: usize
}

impl<'input> Position<'input> {
    fn with_location(c: Chars<'input>) -> Self {
        Position {
            chars: c,
            pos: 0,
            line: 1,
            col: 1
        }
    }
}

impl<'input> Iterator for Position<'input> {
    type Item = (Location, char);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.chars(),
            identation_level: 0,
            line: 1,
            col: 0,
            state: LexerState::LineStart
        }
    }
}