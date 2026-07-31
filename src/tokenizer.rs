#![allow(dead_code)]
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int(u32),
    Float(f32),
    Bool(bool),
    Keyword(Keyword),
    Type(Type),
    Ident(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,

    Add,
    Sub,
    Mul,
    TrueDiv,
    FloorDiv,
    Mod,
    Eq,
    Not,
    Lt,
    Gt,

    EqEq,
    AddEq,
    SubEq,
    MulEq,
    TrueDivEq,
    FloorDivEq,
    ModEq,
    Ne,
    Ge,
    Le,
    Inc,
    Dec,

    LogOr,
    LogAnd,

    BitOr,
    BitAnd,

    At,
    RetType,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Return,
    Let,
    Function,
    While,
    If,
    Else,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U32,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

#[allow(non_snake_case)]
impl LexError {
    pub fn UnexpectedChar(c: char, span: Span) -> Self {
        Self {
            kind: LexErrorKind::UnexpectedChar(c),
            span,
        }
    }
    pub fn InvalidNumber(span: Span) -> Self {
        Self {
            kind: LexErrorKind::InvalidNumber,
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidNumber,
}

#[derive(Clone)]
pub struct Lexer<'src> {
    src: &'src str,
    chars: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn parse(src: &'src str) -> Self {
        Self {
            src,
            chars: src
                .char_indices()
                .peekable(),
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars
            .peek()
            .map(|&(_, c)| c)
    }

    fn pos(&mut self) -> usize {
        self.chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.src.len())
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char()
            && (c == ' ' || c == '\n')
        {
            self.consume()
        }
    }
    fn consume(&mut self) {
        self.chars.next();
    }

    fn consume_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while let Some(c) = self.peek_char() {
            if pred(c) { self.consume() } else { return }
        }
    }

    fn consume2(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek_char()
            && c == expected
        {
            self.consume();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) -> Option<Result<Token, LexError>> {
        self.skip_whitespace();
        let (start, c) = self.bump()?;
        match c {
            c if c.is_alphabetic() => Some(self.lex_ident_or_keyword_or_type(start)),
            c if c.is_ascii_digit() => Some(self.lex_number(start)),
            _ => Some(self.lex_op(c, start)),
        }
    }

    fn lex_ident_or_keyword_or_type(&mut self, start: usize) -> Result<Token, LexError> {
        self.consume_while(|c| c.is_alphanumeric() || c == '_');
        let end = self.pos();
        let text = &self.src[start..end];
        let kind = match text {
            "let" => TokenKind::Keyword(Keyword::Let),
            "return" => TokenKind::Keyword(Keyword::Return),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "while" => TokenKind::Keyword(Keyword::While),
            "function" => TokenKind::Keyword(Keyword::Function),
            "u32" => TokenKind::Type(Type::U32),
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            _ => TokenKind::Ident(String::from(text)),
        };
        Ok(Token {
            kind,
            span: Span { start, end },
        })
    }

    fn lex_number(&mut self, start: usize) -> Result<Token, LexError> {
        self.consume_while(|c| c.is_ascii_digit() || c == '.');
        let end = self.pos();
        let text = &self.src[start..end];
        let span = Span { start, end };
        if let Ok(n) = text.parse() {
            Ok(Token {
                kind: TokenKind::Int(n),
                span,
            })
        } else if let Ok(n) = text.parse() {
            Ok(Token {
                kind: TokenKind::Float(n),
                span,
            })
        } else {
            Err(LexError::InvalidNumber(span))
        }
    }

    fn lex_op(&mut self, c: char, start: usize) -> Result<Token, LexError> {
        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ';' => TokenKind::Semi,
            '+' => {
                if self.consume2('=') {
                    TokenKind::AddEq
                } else if self.consume2('+') {
                    TokenKind::Inc
                } else {
                    TokenKind::Add
                }
            }
            '-' => {
                if self.consume2('=') {
                    TokenKind::SubEq
                } else if self.consume2('-') {
                    TokenKind::Dec
                } else {
                    TokenKind::Sub
                }
            }
            '*' => {
                if self.consume2('=') {
                    TokenKind::MulEq
                } else {
                    TokenKind::Mul
                }
            }
            '/' => {
                if self.consume2('/') {
                    if self.consume2('=') {
                        TokenKind::FloorDivEq
                    } else {
                        TokenKind::FloorDiv
                    }
                } else if self.consume2('=') {
                    TokenKind::TrueDivEq
                } else {
                    TokenKind::TrueDiv
                }
            }
            '%' => {
                if self.consume2('=') {
                    TokenKind::ModEq
                } else {
                    TokenKind::Mod
                }
            }
            '=' => {
                if self.consume2('=') {
                    TokenKind::EqEq
                } else {
                    TokenKind::Eq
                }
            }
            '>' => {
                if self.consume2('=') {
                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }
            '<' => {
                if self.consume2('=') {
                    TokenKind::Le
                } else {
                    TokenKind::Lt
                }
            }
            '!' => {
                if self.consume2('!') {
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }
            '@' => TokenKind::At,
            '|' => {
                if self.consume2('|') {
                    TokenKind::LogOr
                } else {
                    TokenKind::BitOr
                }
            }
            '&' => {
                if self.consume2('&') {
                    TokenKind::LogAnd
                } else {
                    TokenKind::BitAnd
                }
            }
            ',' => {
                self.consume();
                TokenKind::Comma
            }
            '~' if self.consume2('>') => TokenKind::RetType,
            _ => {
                return Err(LexError::UnexpectedChar(
                    c,
                    Span {
                        start,
                        end: self.pos(),
                    },
                ));
            }
        };

        let end = self.pos();
        let span = Span { start, end };
        Ok(Token { kind, span })
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
