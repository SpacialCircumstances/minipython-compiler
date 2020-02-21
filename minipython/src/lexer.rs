use std::str::Chars;
use crate::lexer::Token::*;
use std::iter::Peekable;
use crate::lexer::LexerErrorKind::Unrecognized;
use std::fmt::{Display, Formatter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token<'a> {
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
    NotEqualZero,
    PlusEqualOne,
    MinusEqualOne,
    Equal,
}

impl<'input> Display for Token<'input> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Token<'a> {
    fn from_lexeme(lexeme: &'a str) -> Token {
        match lexeme {
            "while" => While,
            "input" => Input,
            "output" => Output,
            "def" => Def,
            "return" => Return,
            _ => Name(lexeme)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Location {
    line: usize,
    col: usize,
    pos: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Line: {}, Col: {}", self.line, self.col)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LexerErrorKind {
    TabIndent,
    Unrecognized,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LexerError {
    position: Location,
    kind: LexerErrorKind,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.kind {
            LexerErrorKind::TabIndent => write!(f, "Tab indentation not allowed in: {}", self.position),
            LexerErrorKind::Unrecognized => write!(f, "Unrecognized token in: {}", self.position)
        }
    }
}

impl LexerError {
    fn new(position: Location, kind: LexerErrorKind) -> Self {
        LexerError {
            kind,
            position,
        }
    }
}

type LexerResult<'input> = Spanned<Token<'input>, Location, LexerError>;

pub struct Lexer<'input> {
    chars: Peekable<Chars<'input>>,
    input: &'input str,
    last_indent_level: i32,
    indent_level: i32,
    parse_indent: bool,
    line: usize,
    pos: usize,
    col: usize,
    indent_count: u32,
    buffer: Vec<LexerResult<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            input,
            indent_level: 0,
            last_indent_level: 0,
            line: 1,
            pos: 0,
            col: 1,
            parse_indent: true,
            indent_count: 0,
            buffer: Vec::new(),
        }
    }

    fn current_pos(&self) -> Location {
        Location {
            pos: self.pos,
            line: self.line,
            col: self.col,
        }
    }

    fn incr_line(&mut self) {
        self.line += 1;
        self.col = 1;
        self.pos += 1;
        self.parse_indent = true;
        self.last_indent_level = self.indent_level;
        self.indent_level = 0;
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

    fn handle_indent(&mut self) {
        self.parse_indent = false;
        let indent_diff = self.indent_level - self.last_indent_level;
        if indent_diff != 0 {
            let token_count = indent_diff.abs() / 4;
            let tk = if indent_diff < 0 {
                self.indent_count -= token_count as u32;
                Unindent
            } else {
                self.indent_count += token_count as u32;
                Indent
            };
            for _ in 0..token_count {
                self.buffer.push(Ok((self.current_pos(), tk.clone(), self.current_pos())))
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            None => None,
            Some('\n') => {
                self.incr_line();
                Some('\n')
            }
            Some(x) => {
                self.incr_pos();
                Some(x)
            }
        }
    }

    fn not_eq_zero(&mut self) -> Option<LexerResult<'input>> {
        let start = self.current_pos();
        self.incr_pos();
        match self.advance() {
            Some('=') => {
                while let Some(' ') = self.chars.peek() {
                    self.advance();
                }
                match self.advance() {
                    Some('0') => {
                        Some(Ok((start, NotEqualZero, self.current_pos())))
                    }
                    Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
                    None => None
                }
            }
            Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
            None => None
        }
    }

    fn plus_eq_one(&mut self) -> Option<LexerResult<'input>> {
        let start = self.current_pos();
        self.incr_pos();
        match self.advance() {
            Some('=') => {
                while let Some(' ') = self.chars.peek() {
                    self.advance();
                }
                match self.advance() {
                    Some('1') => {
                        Some(Ok((start, PlusEqualOne, self.current_pos())))
                    }
                    Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
                    None => None
                }
            }
            Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
            None => None
        }
    }

    fn minus_eq_one(&mut self) -> Option<LexerResult<'input>> {
        let start = self.current_pos();
        self.incr_pos();
        match self.advance() {
            Some('=') => {
                while let Some(' ') = self.chars.peek() {
                    self.advance();
                }
                match self.advance() {
                    Some('1') => {
                        Some(Ok((start, MinusEqualOne, self.current_pos())))
                    }
                    Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
                    None => None
                }
            }
            Some(_) => Some(Err(LexerError::new(start, Unrecognized))),
            None => None
        }
    }
}

fn single_char_token<'a>(c: char) -> Option<Token<'a>> {
    match c {
        ':' => Some(Colon),
        ',' => Some(Comma),
        '(' => Some(OpenParen),
        ')' => Some(CloseParen),
        '=' => Some(Equal),
        _ => None
    }
}

fn is_separator(c: char) -> bool {
    match c {
        ':' | ',' | ' ' | '\n' | '\t' | '\r' | '(' | ')' | '!' | '+' | '-' | '#' | '=' => true,
        _ => false
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerResult<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.buffer.is_empty() {
                break Some(self.buffer.pop().unwrap());
            } else {
                if self.parse_indent {
                    match self.chars.peek() {
                        None => (),
                        Some(' ') => (),
                        Some(_) => {
                            self.handle_indent();
                        }
                    }
                    if !self.buffer.is_empty() {
                        break Some(self.buffer.pop().unwrap());
                    }
                }
                match self.chars.next() {
                    None => {
                        let missing_unindents = self.indent_count;
                        if missing_unindents > 0 {
                            for _ in 0..(self.indent_count) {
                                self.buffer.push(Ok((self.current_pos(), Unindent, self.current_pos())))
                            }
                        } else {
                            break None
                        }
                    },
                    Some(' ') => {
                        if self.parse_indent {
                            self.indent_level += 1;
                        }
                        self.incr_pos();
                    }
                    Some(c) => {
                        match c {
                            '\n' => {
                                self.incr_line();
                            }

                            '\t' => {
                                let pos = self.current_pos();
                                self.incr_pos();
                                break Some(Err(LexerError::new(pos, LexerErrorKind::TabIndent)));
                            }
                            '\r' => self.incr_pos(),
                            '#' => {
                                self.incr_pos();
                                self.comment()
                            }
                            '!' => {
                                break self.not_eq_zero();
                            }
                            '+' => {
                                break self.plus_eq_one();
                            }
                            '-' => {
                                break self.minus_eq_one();
                            }
                            _ => {
                                let pos = self.current_pos();
                                self.incr_pos();
                                match single_char_token(c) {
                                    Some(tk) => {
                                        break Some(Ok((pos, tk, self.current_pos())));
                                    }
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
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, Lexer};
    use crate::lexer::Token::*;

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
            NotEqualZero,
            Colon
        ];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_2() {
        let code = "return x";
        let tokens = vec![Token::Return, Name("x")];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_3() {
        let code = "a += 1 #test";
        let tokens = vec![Name("a"), PlusEqualOne];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_4() {
        let code = "input: a, b, c";
        let tokens = vec![Input, Colon, Name("a"), Comma, Name("b"), Comma, Name("c")];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_5() {
        let code = "def a(b, c, d): a += 1";
        let tokens = vec![Def, Name("a"), OpenParen, Name("b"), Comma, Name("c"), Comma, Name("d"), CloseParen, Colon, Name("a"), PlusEqualOne];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_6() {
        let code = "    a += 1";
        let tokens = vec![Indent, Name("a"), PlusEqualOne, Unindent];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_7() {
        let code =
            "def test(a, b):
    a += 1
    b -= 1
    #now return the stuff
    return a
c += 1
";
        let tokens =
            vec![Def, Name("test"), OpenParen, Name("a"), Comma, Name("b"), CloseParen, Colon,
                 Indent, Name("a"), PlusEqualOne, Name("b"), MinusEqualOne, Return, Name("a"), Unindent, Name("c"), PlusEqualOne];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_8() {
        let code =
            "a
    b
        c #test
    #asdf
d";
        let tokens = vec![Name("a"), Indent, Name("b"), Indent, Name("c"), Unindent, Unindent, Name("d")];
        lex_equal(code, tokens);
    }

    #[test]
    fn test_lexer_9() {
        let code =
            "def add(a, b):
    while a != 0:
        a-=1
        b += 1
    return b
x+=1
y+=1
z=add(x, y)
z2 = add(a, c)";
        let tokens = vec![Def, Name("add"), OpenParen, Name("a"), Comma, Name("b"), CloseParen, Colon, Indent,
                          While, Name("a"), NotEqualZero, Colon, Indent, Name("a"), MinusEqualOne, Name("b"), PlusEqualOne, Unindent, Return, Name("b"),
                          Unindent, Name("x"), PlusEqualOne, Name("y"), PlusEqualOne, Name("z"), Equal, Name("add"), OpenParen, Name("x"), Comma, Name("y"),
                          CloseParen, Name("z2"), Equal, Name("add"), OpenParen, Name("a"), Comma, Name("c"), CloseParen];
        lex_equal(code, tokens);
    }
}