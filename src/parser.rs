use crate::{
    lexer::{Lexeme, Lexer, Token, TokenKind},
    CalfErr, Pos,
};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{fmt::Debug, str::FromStr};

#[derive(Debug)]
/// Syntactic unit.
pub enum Syntagma<T> {
    Number(T),
    Identifier(String),
    List(Vec<T>),
    Group {
        expr: Box<Expr<T>>,
    },
    UnaryOp {
        op: TokenKind,
        child: Box<Expr<T>>,
    },
    BinaryOp {
        op: TokenKind,
        left_child: Box<Expr<T>>,
        right_child: Box<Expr<T>>,
    },
    TernaryOp {
        left_child: Box<Expr<T>>,
        mid_child: Box<Expr<T>>,
        right_child: Box<Expr<T>>,
    },
    Call {
        func: String,
        args: Vec<Expr<T>>,
    },
    Func {
        params: Vec<String>,
        body: Box<Expr<T>>,
    },
    Empty,
}

#[derive(Debug)]
/// Expression.
pub struct Expr<T> {
    pub syn: Syntagma<T>,
    pub pos: Pos,
}

impl<T> Expr<T> {
    pub fn new(unit: Syntagma<T>, pos: Pos) -> Self {
        Self { syn: unit, pos }
    }
}

#[derive(Debug)]
/// Statement.
pub enum Stmt<T> {
    Assign { name: String, value: Expr<T> },
    Expr(Expr<T>),
}

pub struct Parser<T> {
    tokens: VecDeque<Token<T>>,
    lexer: Lexer,
}

impl<T> Parser<T>
where
    T: FromStr + Debug + PartialEq,
    <T as FromStr>::Err: Debug,
{
    pub fn new(code: &'static str) -> Self {
        Self {
            tokens: Default::default(),
            lexer: Lexer::new(code),
        }
    }

    pub fn scan_stmt(&mut self) -> Result<Stmt<T>, CalfErr> {
        let stmt = self.statement()?;
        Ok(stmt)
    }

    fn statement(&mut self) -> Result<Stmt<T>, CalfErr> {
        if self.is_token(TokenKind::Ident, 0) && self.is_token(TokenKind::Assign, 1) {
            self.assign_statement()
        } else {
            // Otherwise, expression statement
            self.expression_statement()
        }
    }

    fn assign_statement(&mut self) -> Result<Stmt<T>, CalfErr> {
        let ident = self.token().unwrap();
        self.token(); // =
        let value = self.expression()?;
        if let Lexeme::Ident(name) = ident.lexeme {
            Ok(Stmt::Assign { name, value })
        } else {
            Err(CalfErr {
                message: "Invalid identifier for var definition".into(),
                pos: ident.pos,
            })
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt<T>, CalfErr> {
        let expr = self.expression()?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr<T>, CalfErr> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.primary()?;
        while self.is_token(TokenKind::Plus, 0) || self.is_token(TokenKind::Minus, 0) {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.primary()?;
                let pos = expr.pos.clone();
                expr = Expr::new(
                    Syntagma::BinaryOp {
                        op,
                        left_child: Box::new(expr),
                        right_child: Box::new(right),
                    },
                    pos,
                )
            } else {
                return Err(CalfErr {
                    message: "Expected a lexeme of category 'Other'".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr<T>, CalfErr> {
        if self.is_token(TokenKind::Int, 0) || self.is_token(TokenKind::Float, 0) {
            let token = self.token().unwrap();
            if let Lexeme::Number(n) = token.lexeme {
                let expr = Expr::new(Syntagma::Number(n), token.pos);
                return Ok(expr);
            } else {
                return Err(CalfErr {
                    message: "Expected a number".into(),
                    pos: token.pos,
                });
            }
        }
        if self.is_token(TokenKind::Ident, 0) {
            let token = self.token().unwrap();
            if let Lexeme::Ident(id) = token.lexeme {
                let expr = Expr::new(Syntagma::Identifier(id), token.pos);
                return Ok(expr);
            } else {
                return Err(CalfErr {
                    message: "Expected an identifier".into(),
                    pos: token.pos,
                });
            }
        }
        if self.is_token(TokenKind::OpenParenth, 0) {
            self.token(); // consume "("
            let expr = self.expression()?;
            if self.is_token(TokenKind::ClosingParenth, 0) {
                self.token(); // consume ")"
            } else {
                return Err(CalfErr {
                    message: "Expected a closing parenthesis after expression".into(),
                    pos: expr.pos,
                });
            }
            let pos = expr.pos.clone();
            let expr = Expr::new(
                Syntagma::Group {
                    expr: Box::new(expr),
                },
                pos,
            );
            return Ok(expr);
        }
        Err(CalfErr {
            message: "Couldn't parse a valid expression".into(),
            pos: self.tokens[0].pos.clone(),
        })
    }

    fn is_token(&mut self, ttype: TokenKind, offset: usize) -> bool {
        // Get missing tokens from Lexer
        if offset >= self.tokens.len() {
            let missing = offset - self.tokens.len() + 1;
            for _ in 0..missing {
                if let Ok(token) = self.lexer.scan_token() {
                    // Skip this lexeme, not a parseable one
                    if let Lexeme::EOF | Lexeme::None = token.lexeme {
                        continue;
                    }
                    self.tokens.push_back(token);
                } else {
                    // Failed scanning token
                    return false;
                }
            }
        }
        // Check if token exist at the specified offset
        if let Some(token) = self.tokens.get(offset) {
            match token.lexeme {
                Lexeme::Number(_) => ttype == TokenKind::Int || ttype == TokenKind::Float,
                Lexeme::Ident(_) => ttype == TokenKind::Ident,
                Lexeme::Particle(tt) => ttype == tt,
                _ => false,
            }
        } else {
            false
        }
    }

    fn token(&mut self) -> Option<Token<T>> {
        self.tokens.pop_front()
    }

    pub fn is_end(&self) -> bool {
        self.tokens.len() == 0
    }
}

#[derive(Debug)]
/// Abstract Syntax Tree.
pub struct Ast<T> {
    pub statements: Vec<Stmt<T>>,
}

impl<T> Ast<T>
where
    T: FromStr + Debug + PartialEq,
    <T as FromStr>::Err: Debug,
{
    pub fn build(code: &'static str) -> Result<Self, CalfErr> {
        let mut ast = Self {
            statements: Default::default(),
        };
        let mut parser = Parser::new(code);
        loop {
            let stmt = parser.scan_stmt()?;
            ast.statements.push(stmt);
            if parser.is_end() {
                break;
            }
        }
        Ok(ast)
    }
}
