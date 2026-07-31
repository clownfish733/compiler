#![allow(dead_code)]

use crate::tokenizer::{Keyword, LexError, LexErrorKind, Lexer, Span, Token, TokenKind, Type};
use std::collections::HashMap;
use std::iter::Peekable;

pub struct Function {
    ident: String,
    api: FunctionApi,
    block: Block,
}

pub struct FunctionParam {
    ident: String,
    _type: Type,
}

pub struct FunctionApi {
    params: Option<Vec<FunctionParam>>,
    _type: Option<Type>,
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
    Return(Option<Expr>),
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
        ident: String,
        args: Option<Vec<Expr>>,
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
        args: Option<Vec<Expr>>,
    },
}

impl Expr {
    fn insert_left(self, lexp: Expr, op: BinOp) -> Expr {
        Expr::Binary {
            lexp: Box::new(lexp),
            rexp: Box::new(self),
            op,
        }
    }
}

pub enum GeneralType {
    Int(u32),
    Float(f32),
    Bool(bool),
}

#[derive(PartialEq, Debug)]
pub enum Op {
    BinOp(BinOp),
    CaoBinOp(BinOp),
    UnOp(UnOp),
    None,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum UnOp {
    Inc,
    Dec,
}

impl Op {
    pub fn from_kind(kind: &TokenKind) -> Self {
        // ignoring not bit operator for now will fix later
        match kind {
            TokenKind::Add => Op::BinOp(BinOp::Add),
            TokenKind::Sub => Op::BinOp(BinOp::Sub),
            TokenKind::Mul => Op::BinOp(BinOp::Mul),
            TokenKind::TrueDiv => Op::BinOp(BinOp::TrueDiv),
            TokenKind::FloorDiv => Op::BinOp(BinOp::FloorDiv),
            TokenKind::Mod => Op::BinOp(BinOp::Mod),
            TokenKind::EqEq => Op::BinOp(BinOp::EqEq),
            TokenKind::Ne => Op::BinOp(BinOp::Ne),
            TokenKind::Lt => Op::BinOp(BinOp::Lt),
            TokenKind::Gt => Op::BinOp(BinOp::Gt),
            TokenKind::Le => Op::BinOp(BinOp::Le),
            TokenKind::Ge => Op::BinOp(BinOp::Ge),
            TokenKind::BitOr => Op::BinOp(BinOp::BitOr),
            TokenKind::BitAnd => Op::BinOp(BinOp::BitAnd),
            TokenKind::LogOr => Op::BinOp(BinOp::LogOr),
            TokenKind::LogAnd => Op::BinOp(BinOp::LogAnd),
            TokenKind::Inc => Op::UnOp(UnOp::Inc),
            TokenKind::Dec => Op::UnOp(UnOp::Dec),
            TokenKind::AddEq => Op::CaoBinOp(BinOp::Add),
            TokenKind::SubEq => Op::CaoBinOp(BinOp::Sub),
            TokenKind::MulEq => Op::CaoBinOp(BinOp::Mul),
            TokenKind::TrueDivEq => Op::CaoBinOp(BinOp::TrueDiv),
            TokenKind::FloorDivEq => Op::CaoBinOp(BinOp::FloorDiv),
            TokenKind::ModEq => Op::CaoBinOp(BinOp::Mod),
            _ => Op::None,
        }
    }
}
#[derive(Default)]
pub struct FunctionMap(HashMap<String, Function>);

pub struct Parser<'l> {
    tokens: Peekable<Lexer<'l>>,
    functions: FunctionMap,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Option<Span>,
}

impl ParseError {
    pub fn highlight_error(&self, contents: &str) {
        if let Some(span) = &self.span {
            println!("error: {}", &contents[span.start - 4..span.end + 4])
        }
    }
}

#[derive(PartialEq, Debug)]
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
        loop {
            match parser.parse_function() {
                Err(e) if e.kind == ParseErrorKind::UnexpectedEOF => return Ok(parser.functions),
                Err(e) => return Err(e),
                _ => continue,
            }
        }
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
        self.expect_token(TokenKind::Keyword(Keyword::Function))?;
        let ident = self.expect_ident()?;
        let params = self.parse_params()?;

        let _type = {
            if self
                .some_peek_token()?
                .kind
                == TokenKind::RetType
            {
                self.consume()?;
                Some(self.expect_type()?)
            } else {
                None
            }
        };
        let block = self.parse_block()?;

        let function = Function {
            ident: ident.clone(),
            api: FunctionApi { params, _type },
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

    fn parse_params(&mut self) -> Result<Option<Vec<FunctionParam>>, ParseError> {
        self.expect_token(TokenKind::LParen)?;
        if self
            .some_peek_token()?
            .kind
            == TokenKind::RParen
        {
            self.consume()?;
            return Ok(None);
        }
        let mut params = Vec::default();
        loop {
            let ident = self.expect_ident()?;
            let _type = self.expect_type()?;
            params.push(FunctionParam { ident, _type });
            if self
                .some_peek_token()?
                .kind
                == TokenKind::RParen
            {
                self.consume()?;
                break;
            }
            self.expect_token(TokenKind::Comma)?;
        }

        Ok(Some(params))
    }
    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect_token(TokenKind::LBrace)?;
        let mut stmnts = Vec::new();
        loop {
            let token = self.some_next_token()?;
            match token.kind {
                TokenKind::Keyword(Keyword::Let) => stmnts.push(self.parse_let()?),
                TokenKind::Keyword(Keyword::Return) => stmnts.push(self.parse_return()?),
                TokenKind::Keyword(Keyword::While) => stmnts.push(self.parse_while()?),
                TokenKind::Keyword(Keyword::If) => stmnts.push(self.parse_if()?),
                TokenKind::Ident(ident) => stmnts.push(self.parse_ident(ident)?),
                TokenKind::RBrace => return Ok(Block(stmnts)),
                _ => {
                    return Err(ParseError {
                        kind: ParseErrorKind::UnexpectedToken,
                        span: Some(token.span),
                    });
                }
            }
        }
    }

    fn parse_tuple(&mut self, terminator: TokenKind) -> Result<Option<Vec<Expr>>, ParseError> {
        let mut exprs = Vec::default();
        dbg!(self.peek_token());
        loop {
            if self
                .some_peek_token()?
                .kind
                == terminator
            {
                self.consume()?;
                break;
            }
            println!("-----------------pre parse-----------------");
            exprs.push(self.parse_expr()?);
            if self
                .some_peek_token()?
                .kind
                == terminator
            {
                println!("---------proper termination");
                self.consume()?;
                break;
            }
            dbg!(self.peek_token());
            println!("--------------improper termination");
            self.expect_token(TokenKind::Comma)?;
        }

        if exprs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(exprs))
        }
    }
    //parse_keyword always assume indicating keyword has already been confirmed
    fn parse_let(&mut self) -> Result<Stmnt, ParseError> {
        let ident = self.expect_ident()?;
        let _type = self.expect_type()?;
        let init = self.parse_expr()?;
        self.expect_token(TokenKind::Semi)?;
        Ok(Stmnt::Let { ident, _type, init })
    }
    fn parse_return(&mut self) -> Result<Stmnt, ParseError> {
        println!("--------------return------------");
        dbg!(self.peek_token());
        if self
            .some_peek_token()?
            .kind
            == TokenKind::Semi
        {
            self.consume()?;
            return Ok(Stmnt::Return(None));
        }
        let expr = self.parse_expr()?;
        self.expect_token(TokenKind::Semi)?;
        Ok(Stmnt::Return(Some(expr)))
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
        match self
            .some_peek_token()?
            .kind
        {
            TokenKind::LParen => {
                self.consume()?;
                let args = self.parse_tuple(TokenKind::RParen)?;
                Ok(Stmnt::VoidFunction { ident, args })
            }
            TokenKind::Eq => {
                let init = self.parse_expr()?;
                Ok(Stmnt::Assign { ident, init })
            }
            _ => self.parse_cao(ident),
        }
    }
    fn parse_cao(&mut self, ident: String) -> Result<Stmnt, ParseError> {
        let token = self.some_next_token()?;
        if let Op::CaoBinOp(op) = Op::from_kind(&token.kind) {
            Ok(Stmnt::Assign {
                ident: ident.clone(),
                init: self
                    .parse_expr()?
                    .insert_left(Expr::Ident(ident), op),
            })
        } else {
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            })
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        dbg!(self.peek_token());
        let token = self.some_next_token()?;
        let lexp = match token.kind {
            TokenKind::Int(n) => Expr::Lit(GeneralType::Int(n)),
            TokenKind::Float(n) => Expr::Lit(GeneralType::Float(n)),
            TokenKind::Bool(n) => Expr::Lit(GeneralType::Bool(n)),
            TokenKind::Ident(ident) => {
                if self
                    .some_peek_token()?
                    .kind
                    == TokenKind::RParen
                {
                    self.consume()?;
                    let args = self.parse_tuple(TokenKind::RParen)?;
                    Expr::Call { ident, args }
                } else {
                    dbg!(self.peek_token());
                    Expr::Ident(ident)
                }
            }
            TokenKind::LParen => {
                let lexp = self.parse_expr()?;
                self.expect_token(TokenKind::RParen)?;
                lexp
            }
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::UnexpectedToken,
                    span: Some(token.span),
                });
            }
        };

        let t = self.some_peek_token()?;
        match Op::from_kind(&t.kind) {
            Op::CaoBinOp(_) => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(t.span),
            }),
            Op::UnOp(op) => {
                self.consume()?;
                Ok(Expr::Unary {
                    exp: Box::new(lexp),
                    op,
                })
            }
            Op::BinOp(op) => {
                self.consume()?;
                Ok(Expr::Binary {
                    lexp: Box::new(lexp),
                    rexp: Box::new(self.parse_expr()?),
                    op,
                })
            }
            Op::None => Ok(lexp),
        }
    }

    fn parse_bracket_param() {}
}

/*
fn parse_expr(&mut self) -> Result<Expr, ParseError> {
    dbg!(self.peek_token());
    let token = self.some_next_token()?;
    let lexp = match token.kind {
        TokenKind::LParen => self.parse_expr()?,
        TokenKind::Int(n) => Expr::Lit(GeneralType::Int(n)),
        TokenKind::Float(n) => Expr::Lit(GeneralType::Float(n)),
        TokenKind::Bool(n) => Expr::Lit(GeneralType::Bool(n)),
        TokenKind::Ident(ident) => {
            println!("--------------is ident ---------------");
            dbg!(self.peek_token());
            if self
                .some_peek_token()?
                .kind
                == TokenKind::LParen
            {
                self.consume()?;
                let c = Expr::Call {
                    ident,
                    args: Some(self.parse_tuple(TokenKind::RParen)?),
                };
                println!("------------ post func -----------");
                dbg!(self.peek_token());
                c
            } else {
                Expr::Ident(ident)
            }
        }
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                span: Some(token.span),
            });
        }
    };
    dbg!(self.peek_token());

    let t = self.some_peek_token()?;
    match Op::from_kind(&t.kind) {
        Op::CaoBinOp(_) => Err(ParseError {
            kind: ParseErrorKind::UnexpectedToken,
            span: Some(token.span),
        }),
        Op::UnOp(op) => {
            self.consume()?;
            Ok(Expr::Unary {
                exp: Box::new(lexp),
                op,
            })
        }
        Op::BinOp(op) => {
            self.consume()?;
            Ok(lexp.insert_left(self.parse_expr()?, op))
        }
        Op::None => {
            if t.kind == TokenKind::RParen {
                self.consume()?;
            }
            Ok(lexp)
        }
    }
}
*/
