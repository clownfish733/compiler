#![allow(dead_code)]

use crate::tokenizer::{Keyword, LexError, LexErrorKind, Lexer, Span, Token, TokenKind, Type};
use std::collections::HashMap;
use std::iter::Peekable;

pub struct Function {
    ident: String,
    api: FunctionApi,
    block: Block,
}

pub struct FunctionIn {
    ident: String,
    _type: Type,
}

pub struct FunctionOut {
    _type: Type,
}
pub struct FunctionApi {
    ins: Vec<FunctionIn>,
    outs: Vec<FunctionOut>,
}

pub struct Block(Vec<Stmnt>);

pub enum Stmnt {
    Let {
        ident: String,
        _type: Type,
        init: Expr,
    },
    Assign {
        ident: String,
        init: Expr,
    },
    Return(Vec<Expr>),
    While {
        condition: Expr,
        block: Block,
    },
    If {
        condition: Expr,
        if_block: Block,
    },
    IfElse {
        condition: Expr,
        if_block: Block,
        else_block: Block,
    },
    VoidFunction {
        call: Expr,
    },
}

pub enum Expr {
    Binary {
        lexp: Box<Expr>,
        rexp: Box<Expr>,
        op: BinOp,
    },
    Unary {
        exp: Box<Expr>,
        op: UnOp,
    },
    Ident(String),
    Lit(GeneralType),
    Call {
        ident: String,
        args: Vec<Expr>,
    },
}

pub enum GeneralType {
    Int(u64),
    Float(f64),
    Bool(bool),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    TrueDiv,
    FloorDiv,
    Mod,
    EqEq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    BitOr,
    BitAnd,
    LogOr,
    LogAnd,
}

pub enum UnOp {
    Not,
    Inc,
    Dec,
}

#[derive(Default)]
pub struct FunctionMap(HashMap<String, Function>);

pub struct Parser<'l> {
    tokens: Peekable<Lexer<'l>>,
    functions: FunctionMap,
}

pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Span>,
}

#[derive(PartialEq)]
pub enum ParseErrorKind {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidNumber,
    UnexpectedEOF,
    ExpectedFunction,
    UnexpectedToken,
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        let kind = match value.kind {
            LexErrorKind::UnexpectedChar(c) => ParseErrorKind::UnexpectedChar(c),
            LexErrorKind::UnterminatedString => ParseErrorKind::UnterminatedString,
            LexErrorKind::InvalidNumber => ParseErrorKind::InvalidNumber,
        };
        Self {
            kind,
            span: Some(value.span),
        }
    }
}

impl<'l> Parser<'l> {
    pub fn parse(tokens: Lexer<'l>) -> Result<FunctionMap, ParseError> {
        let mut parser = Self {
            tokens: tokens.peekable(),
            functions: FunctionMap::default(),
        };
        while parser.parse_function()? {}
        Ok(parser.functions)
    }

    fn next_token(&mut self) -> Option<Result<Token, LexError>> {
        self.tokens.next()
    }

    fn some_next_token(&mut self) -> Result<Token, ParseError> {
        match self.next_token() {
            Some(t) => match t {
                Ok(t) => Ok(t),
                Err(e) => Err(ParseError::from(e)),
            },
            None => Err(ParseError {
                kind: ParseErrorKind::UnexpectedEOF,
                span: None,
            }),
        }
    }

    fn peek_token(&mut self) -> Option<&Result<Token, LexError>> {
        self.tokens.peek()
    }

    fn some_peek_token(&mut self) -> Result<Token, ParseError> {
        match self.peek_token() {
            Some(res) => match res {
                Ok(t) => Ok(t.clone()),
                Err(e) => Err(ParseError::from(e.clone())),
            },
            None => Err(ParseError {
                kind: ParseErrorKind::UnexpectedEOF,
                span: None,
            }),
        }
    }
    fn consume(&mut self) -> Result<(), ParseError> {
        let _ = self.some_next_token()?;
        Ok(())
    }
    //returns Ok(false) if valid EOF
    fn parse_function(&mut self) -> Result<bool, ParseError> {
        let token = match self.some_next_token() {
            Ok(t) => t,
            Err(e) => {
                if e.kind == ParseErrorKind::UnexpectedEOF {
                    return Ok(false);
                } else {
                    return Err(e);
                }
            }
        };
        match token.kind {
            TokenKind::Keyword(Keyword::Function) => {}
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::UnexpectedToken,
                    span: Some(token.span),
                });
            }
        }
        let ident = self.expect_ident()?;
        let params = self.parse_params()?;
        let returns = self.parse_return_type()?;
        let block = self.parse_block()?;

        let function = Function {
            ident: ident.clone(),
            api: FunctionApi {
                ins: params,
                outs: returns,
            },
            block,
        };
        self.functions
            .0
            .insert(ident, function);

        Ok(true)
    }
    fn expect_ident(&mut self) -> Result<String, ParseError> {
        let token = self.some_next_token()?;
        match token.kind {
            TokenKind::Ident(ident) => Ok(ident),
            _ => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            }),
        }
    }

    fn expect_type(&mut self) -> Result<Type, ParseError> {
        self.expect_token(TokenKind::At)?;
        let token = self.some_next_token()?;
        match token.kind {
            TokenKind::Type(_type) => Ok(_type),
            _ => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            }),
        }
    }
    fn expect_token(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        let token = self.some_next_token()?;
        if token.kind == kind {
            Ok(())
        } else {
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            })
        }
    }

    fn parse_params(&mut self) -> Result<Vec<FunctionIn>, ParseError> {
        self.expect_token(TokenKind::LParen)?;
        let mut params = Vec::default();
        loop {
            let token = self.some_next_token()?;
            match token.kind {
                TokenKind::RParen => return Ok(params),
                TokenKind::Ident(ident) => {
                    let _type = self.expect_type()?;
                    params.push(FunctionIn { ident, _type });
                    let token = self.some_next_token()?;
                    match token.kind {
                        TokenKind::Comma => continue,
                        TokenKind::RParen => return Ok(params),
                        _ => {
                            return Err(ParseError {
                                kind: ParseErrorKind::UnexpectedToken,
                                span: Some(token.span),
                            });
                        }
                    }
                }
                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken,
                        span: Some(token.span),
                    });
                }
            }
        }
    }

    fn parse_return_type(&mut self) -> Result<Vec<FunctionOut>, ParseError> {
        let mut outs = Vec::default();
        if self
            .some_peek_token()?
            .kind
            == TokenKind::LBrace
        {
            return Ok(outs);
        }
        self.expect_token(TokenKind::RetType)?;
        loop {
            outs.push(FunctionOut {
                _type: self.expect_type()?,
            });
            if self
                .some_peek_token()?
                .kind
                == TokenKind::LBrace
            {
                return Ok(outs);
            }
            self.expect_token(TokenKind::Comma)?;
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect_token(TokenKind::RParen)?;
        let mut stmnts = Vec::new();
        loop {
            let token = self.some_next_token()?;
            match token.kind {
                TokenKind::Keyword(Keyword::Let) => stmnts.push(self.parse_let()?),
                TokenKind::Keyword(Keyword::Return) => stmnts.push(self.parse_return()?),
                TokenKind::Keyword(Keyword::While) => stmnts.push(self.parse_while()?),
                TokenKind::Keyword(Keyword::If) => stmnts.push(self.parse_if()?),
                TokenKind::Ident(ident) => stmnts.push(self.parse_ident(ident)?),
                TokenKind::LBrace => return Ok(Block(stmnts)),
                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken,
                        span: Some(token.span),
                    });
                }
            }
        }
    }

    fn parse_tuple(&mut self, terminator: TokenKind) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::default();
        loop {
            if self
                .some_next_token()?
                .kind
                == terminator
            {
                break;
            }

            exprs.push(self.parse_expr()?);
            if self
                .some_next_token()?
                .kind
                == terminator
            {
                self.consume()?;
                break;
            }

            self.expect_token(TokenKind::Comma)?;
        }
        Ok(exprs)
    }
    //parse_keyword always assume indicating keyword has already been confirmend and consumed
    fn parse_let(&mut self) -> Result<Stmnt, ParseError> {
        let ident = self.expect_ident()?;
        let _type = self.expect_type()?;
        let init = self.parse_expr()?;
        self.expect_token(TokenKind::Semi)?;
        Ok(Stmnt::Let { ident, _type, init })
    }
    fn parse_return(&mut self) -> Result<Stmnt, ParseError> {
        let exprs = self.parse_tuple(TokenKind::Semi)?;
        Ok(Stmnt::Return(exprs))
    }
    fn parse_while(&mut self) -> Result<Stmnt, ParseError> {
        let condition = self.parse_expr()?;
        let block = self.parse_block()?;
        Ok(Stmnt::While { condition, block })
    }
    fn parse_if(&mut self) -> Result<Stmnt, ParseError> {
        let condition = self.parse_expr()?;
        let if_block = self.parse_block()?;
        if self
            .some_peek_token()?
            .kind
            == TokenKind::Keyword(Keyword::Else)
        {
            self.consume()?;
            let else_block = self.parse_block()?;
            Ok(Stmnt::IfElse {
                condition,
                if_block,
                else_block,
            })
        } else {
            Ok(Stmnt::If {
                condition,
                if_block,
            })
        }
    }
    fn parse_ident(&mut self, ident: String) -> Result<Stmnt, ParseError> {
        let token = self.some_next_token()?;
        match token.kind {
            TokenKind::LParen => {
                todo!()
            }
            TokenKind::Eq => {
                let init = self.parse_expr()?;
                Ok(Stmnt::Assign { ident, init })
            }
            _ => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            }),
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut buf: Option<Token> = None;
        match self
            .some_next_token()?
            .kind
        {
            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                self.expect_token(TokenKind::RParen)?;
            }
            _ => todo!(),
        }
        Ok(Expr::Lit(GeneralType::Int(4)))
    }
