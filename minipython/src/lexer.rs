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
    chars: Position<'input>,
    identation_level: i32,
    state: LexerState
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: Position::from_chars(input.chars()),
            identation_level: 0,
            state: LexerState::LineStart
        }
    }
}