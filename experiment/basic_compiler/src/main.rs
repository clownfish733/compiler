use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Op(Op),
    Lit(u32),
}

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn eval(&self, left: u32, right: u32) -> u32 {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Div => left / right,
            Op::Mul => left * right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

#[derive(Debug, Clone)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug)]
struct Lexer<'src> {
    src: &'src str,
    chars: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {
    fn parse(src: &'src str) -> Self {
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

    fn consume_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while let Some(c) = self.peek_char() {
            if pred(c) { self.consume() } else { return }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let (start, c) = self.bump()?;
        let kind = match c {
            c if c.is_ascii_digit() => return Some(self.lex_number(start)),
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '+' => TokenKind::Op(Op::Add),
            '-' => TokenKind::Op(Op::Sub),
            '*' => TokenKind::Op(Op::Mul),
            '/' => TokenKind::Op(Op::Div),
            _ => panic!(),
        };
        Some(Token {
            kind,
            span: Span {
                start,
                end: self.pos(),
            },
        })
    }

    fn lex_number(&mut self, start: usize) -> Token {
        self.consume_while(|c| c.is_ascii_digit());
        let end = self.pos();
        Token {
            kind: TokenKind::Lit(
                self.src[start..end]
                    .parse()
                    .unwrap(),
            ),
            span: Span { start, end },
        }
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

    fn consume(&mut self) {
        self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char()
            && (c == ' ' || c == '\n')
        {
            self.consume()
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Clone)]
enum Expr {
    Binary {
        lexp: Box<Expr>,
        rexp: Box<Expr>,
        op: Op,
    },
    Lit(u32),
}

#[derive(Debug)]
struct Parser<'l> {
    tokens: Peekable<Lexer<'l>>,
}

#[derive(Debug)]
enum ParseError {
    UnexpectedChar,
    UnexpectedEof,
}

impl<'l> Parser<'l> {
    fn parse(tokens: Lexer<'l>) -> Result<Expr, ParseError> {
        let mut parser = Parser {
            tokens: tokens.peekable(),
        };
        parser.parse_expr()
    }
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = match self.next_token() {
            Some(token) => token,
            None => return Err(ParseError::UnexpectedEof),
        };
        let lexp = match token.kind {
            TokenKind::LParen => self.parse_expr()?,
            TokenKind::Lit(n) => Expr::Lit(n),
            _ => {
                dbg!(self);
                return Err(ParseError::UnexpectedChar);
            }
        };
        let token = match self.next_token() {
            Some(t) => t,
            None => return Ok(lexp),
        };
        match token.kind {
            TokenKind::RParen => Ok(lexp),
            TokenKind::Op(op) => Ok(Expr::Binary {
                lexp: Box::new(lexp),
                rexp: Box::new(self.parse_expr()?),
                op,
            }),
            _ => {
                dbg!(self);
                Err(ParseError::UnexpectedChar)
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}

fn shitty_eval(ast: Expr) -> u32 {
    match ast {
        Expr::Lit(n) => n,
        Expr::Binary { lexp, rexp, op } => op.eval(shitty_eval(*lexp), shitty_eval(*rexp)),
    }
}

#[derive(Default)]
struct Evaluator {
    stack: Vec<u32>,
}

impl Evaluator {
    fn eval(ast: Expr) -> u32 {
        let mut evaluator = Evaluator::default();
        evaluator.eval_expr(ast);
        evaluator
            .stack
            .pop()
            .unwrap()
    }

    fn eval_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Lit(n) => self.stack.push(n),
            Expr::Binary { lexp, rexp, op } => {
                self.eval_expr(*lexp);
                self.eval_expr(*rexp);
                let a = self
                    .stack
                    .pop()
                    .unwrap();
                let b = self
                    .stack
                    .pop()
                    .unwrap();
                self.stack
                    .push(match op {
                        Op::Add => a + b,
                        Op::Sub => a - b,
                        Op::Mul => a * b,
                        Op::Div => a / b,
                    })
            }
        }
    }
}

fn get_file_path() -> Option<String> {
    let mut args = std::env::args();
    if let Some(_) = args.next()
        && let Some(filepath) = args.next()
    {
        Some(filepath)
    } else {
        None
    }
}

fn get_contents(path: &String) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn arg_error_message() {
    eprintln!("must be run like: cargo run $file_path")
}

fn path_error_message(path: &String) {
    eprintln!("Invalid file path: {}", path);
}

fn main() {
    let path = match get_file_path() {
        Some(path) => path,
        None => {
            arg_error_message();
            return;
        }
    };
    let contents = match get_contents(&path) {
        Some(contents) => contents,
        None => {
            path_error_message(&path);
            return;
        }
    };

    let tokens = Lexer::parse(&contents);
    let ast = match Parser::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    assert_eq!(shitty_eval(ast.clone()), Evaluator::eval(ast));
}
