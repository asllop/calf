use crate::{
    common::{CalfErr, Pos},
    lexer::{Lexeme, Lexer, Token, TokenKind},
};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{fmt::Debug, str::FromStr};

//TODO: create an AST struct with a Vec<Expr>, and use indexes to this vec instead of Box<Expr> to reduce allocations.

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
    Lambda {
        params: Vec<String>,
        body: Box<Expr<T>>,
    },
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
        if self.is_token(TokenKind::Ident, 0)? && self.is_token(TokenKind::Assign, 1)? {
            self.assign_statement()
        } else {
            // Otherwise, expression statement
            self.expression_statement()
        }
    }

    fn assign_statement(&mut self) -> Result<Stmt<T>, CalfErr> {
        let ident = self.token().unwrap();
        self.token(); // Consume "="
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
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.comparison()?;
        while self.is_token(TokenKind::TwoEquals, 0)? || self.is_token(TokenKind::NotEqual, 0)? {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.comparison()?;
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
                    message: "Expected a particle to parse equality".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.logic()?;
        while self.is_token(TokenKind::GreaterThan, 0)?
            || self.is_token(TokenKind::LesserThan, 0)?
            || self.is_token(TokenKind::GtEqual, 0)?
            || self.is_token(TokenKind::LtEqual, 0)?
            || self.is_token(TokenKind::TwoAnds, 0)?
            || self.is_token(TokenKind::TwoOrs, 0)?
        {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.logic()?;
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
                    message: "Expected a particle to parse comparison".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn logic(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.term()?;
        while self.is_token(TokenKind::And, 0)? || self.is_token(TokenKind::Or, 0)? {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.term()?;
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
                    message: "Expected a particle to parse logic".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.factor()?;
        while self.is_token(TokenKind::Plus, 0)? || self.is_token(TokenKind::Minus, 0)? {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.factor()?;
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
                    message: "Expected a particle to parse term".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.unary()?;
        while self.is_token(TokenKind::Star, 0)?
            || self.is_token(TokenKind::Slash, 0)?
            || self.is_token(TokenKind::Percent, 0)?
        {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.unary()?;
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
                    message: "Expected a particle to parse comparison".into(),
                    pos: op.pos,
                });
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<T>, CalfErr> {
        if self.is_token(TokenKind::Not, 0)? || self.is_token(TokenKind::Minus, 0)? {
            let op = self.token().unwrap();
            if let Lexeme::Particle(op) = op.lexeme {
                let right = self.unary()?;
                let pos = right.pos.clone();
                return Ok(Expr::new(
                    Syntagma::UnaryOp {
                        op,
                        child: Box::new(right),
                    },
                    pos,
                ));
            } else {
                return Err(CalfErr {
                    message: "Expected a particle to parse unary".into(),
                    pos: op.pos,
                });
            }
        }
        self.primary()
    }

    //TODO: parse "#" and "." operators

    fn primary(&mut self) -> Result<Expr<T>, CalfErr> {
        //TODO: parse list literals
        if self.is_token(TokenKind::Int, 0)? || self.is_token(TokenKind::Float, 0)? {
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
        if self.is_token(TokenKind::Ident, 0)? {
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
        if self.is_token(TokenKind::OpenParenth, 0)? {
            self.token(); // consume "("
            let expr = self.expression()?;
            if self.is_token(TokenKind::ClosingParenth, 0)? {
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
        //TODO: check the next token and see if we can provide a more specific err msg
        // If we are here there is something badly formed
        Err(CalfErr {
            message: "Couldn't parse a valid expression".into(),
            pos: self.lexer.pos(),
        })
    }

    fn is_token(&mut self, ttype: TokenKind, offset: usize) -> Result<bool, CalfErr> {
        // Get missing tokens from Lexer
        if offset >= self.tokens.len() {
            let missing = offset - self.tokens.len() + 1;
            for _ in 0..missing {
                let token = self.lexer.scan_token()?;
                // Skip this lexeme, not a parseable one
                if let Lexeme::EOF | Lexeme::None = token.lexeme {
                    continue;
                }
                self.tokens.push_back(token);
            }
        }
        // Check if token exist at the specified offset
        if let Some(token) = self.tokens.get(offset) {
            Ok(match token.lexeme {
                Lexeme::Number(_) => ttype == TokenKind::Int || ttype == TokenKind::Float,
                Lexeme::Ident(_) => ttype == TokenKind::Ident,
                Lexeme::Particle(tt) => ttype == tt,
                _ => false,
            })
        } else {
            Ok(false)
        }
    }

    fn token(&mut self) -> Option<Token<T>> {
        self.tokens.pop_front()
    }

    pub fn is_end(&self) -> bool {
        self.tokens.len() == 0
    }
}
