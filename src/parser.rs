use crate::{
    common::{CalfErr, Pos},
    lexer::{FromToken, Lexeme, Lexer, Token, TokenKind},
};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{fmt::Debug, str::FromStr};

//TODO: create a Vec<Expr<T>>, and use indexes to this vec instead of Box<Expr<T>> to reduce allocations.

#[derive(Debug)]
/// Syntactic unit.
pub enum Syntagma<T> {
    Number(T),
    Identifier(String),
    Vector(Vec<T>),
    Range {
        init: T,
        len: u64,
        step: T,
    },
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

pub struct Parser<'a, T> {
    tokens: VecDeque<Token<T>>,
    lexer: Lexer<'a>,
}

impl<'a, T> Parser<'a, T>
where
    T: FromStr + Debug + PartialEq,
    <T as FromStr>::Err: Debug,
{
    pub fn new(code: &'a str) -> Self {
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
        self.token().unwrap(); // Consume "="
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
        self.ternay()
    }

    // Parsing a ternay expression:
    //      cond_expr ? then_expr : else_expr
    // Is equivalent to parsing two nested binary expressions:
    //      cond_expr ? (then_expr : else_expr)
    fn ternay(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut cond_expr = self.equality()?;
        if self.is_token(TokenKind::Question, 0)? {
            self.token().into_particle()?;
            // Parse colon part of the expression
            let right_expr = |_self: &mut Self| -> Result<Expr<T>, CalfErr> {
                let mut then_expr = _self.ternay()?;
                if _self.is_token(TokenKind::Colon, 0)? {
                    let (colon_op, _) = _self.token().into_particle()?;
                    let else_expr = _self.ternay()?;
                    let then_pos = then_expr.pos.clone();
                    then_expr = Expr::new(
                        Syntagma::BinaryOp {
                            op: colon_op,
                            left_child: Box::new(then_expr),
                            right_child: Box::new(else_expr),
                        },
                        then_pos,
                    )
                } else {
                    return Err(CalfErr {
                        message: "Expected a colon operator".into(),
                        pos: then_expr.pos,
                    });
                }
                Ok(then_expr)
            }(self)?;

            let pos_cond = cond_expr.pos.clone();

            if let Syntagma::BinaryOp {
                op: TokenKind::Colon,
                left_child,
                right_child,
            } = right_expr.syn
            {
                cond_expr = Expr::new(
                    Syntagma::TernaryOp {
                        left_child: Box::new(cond_expr),
                        mid_child: left_child,
                        right_child,
                    },
                    pos_cond,
                )
            } else {
                return Err(CalfErr {
                    message: "Ternary operator '?' expects a colon operator".into(),
                    pos: pos_cond,
                });
            }
        }
        Ok(cond_expr)
    }

    fn equality(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.comparison()?;
        while self.is_token(TokenKind::TwoEquals, 0)? || self.is_token(TokenKind::NotEqual, 0)? {
            let (op, _) = self.token().into_particle()?;
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
            let (op, _) = self.token().into_particle()?;
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
        }
        Ok(expr)
    }

    fn logic(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.term()?;
        while self.is_token(TokenKind::And, 0)? || self.is_token(TokenKind::Or, 0)? {
            let (op, _) = self.token().into_particle()?;
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
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.factor()?;
        while self.is_token(TokenKind::Plus, 0)? || self.is_token(TokenKind::Minus, 0)? {
            let (op, _) = self.token().into_particle()?;
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
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<T>, CalfErr> {
        let mut expr = self.unary()?;
        while self.is_token(TokenKind::Star, 0)?
            || self.is_token(TokenKind::Slash, 0)?
            || self.is_token(TokenKind::Percent, 0)?
        {
            let (op, _) = self.token().into_particle()?;
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
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<T>, CalfErr> {
        if self.is_token(TokenKind::Not, 0)? || self.is_token(TokenKind::Minus, 0)? {
            let (op, _) = self.token().into_particle()?;
            let right = self.unary()?;
            let pos = right.pos.clone();
            return Ok(Expr::new(
                Syntagma::UnaryOp {
                    op,
                    child: Box::new(right),
                },
                pos,
            ));
        }
        self.call()
    }

    //TODO: parse "." operator

    //TODO: parse indexations: set and slice

    //TODO: parse "#" operator

    fn call(&mut self) -> Result<Expr<T>, CalfErr> {
        if self.is_token(TokenKind::Ident, 0)? && self.is_token(TokenKind::OpenCurly, 1)? {
            let (func, pos) = self.token().into_ident()?;
            self.token().into_particle()?; // consume "{"
            let mut args = vec![];
            let mut expect_comma = false;
            let mut expect_arg = true;
            loop {
                if self.is_token(TokenKind::ClosingCurly, 0)? {
                    self.token().into_particle()?; // consume "}"
                    break;
                }

                if expect_comma {
                    if self.is_token(TokenKind::Comma, 0)? {
                        self.token().into_particle()?; // consume ","
                        expect_arg = true;
                        expect_comma = false;
                        continue;
                    } else {
                        let (_, pos) = self.token().into_parts()?;
                        return Err(CalfErr {
                            message: "Expecting a comma".into(),
                            pos,
                        });
                    }
                } else if self.is_token(TokenKind::Comma, 0)? {
                    let (_, pos) = self.token().into_parts()?;
                    return Err(CalfErr {
                        message: "Not expecting a comma".into(),
                        pos,
                    });
                }

                if expect_arg {
                    let arg = self.expression()?;
                    args.push(arg);
                    expect_arg = false;
                    expect_comma = true;
                }
            }

            return Ok(Expr::new(Syntagma::Call { func, args }, pos));
        }
        self.lambda()
    }

    fn lambda(&mut self) -> Result<Expr<T>, CalfErr> {
        if self.is_ident("fn", 0)? && self.is_token(TokenKind::OpenParenth, 1)? {
            let (_, pos) = self.token().into_ident()?; // consume "fn"
            self.token().into_particle()?; // consume "("

            let mut params = vec![];
            let mut expect_comma = false;
            let mut expect_param = true;

            loop {
                if self.is_token(TokenKind::ClosingParenth, 0)? {
                    self.token().into_particle()?; // consume ")"
                    break;
                }

                if expect_comma {
                    if self.is_token(TokenKind::Comma, 0)? {
                        self.token().into_particle()?; // consume ","
                        expect_param = true;
                        expect_comma = false;
                        continue;
                    } else {
                        let (_, pos) = self.token().into_parts()?;
                        return Err(CalfErr {
                            message: "Expecting a comma".into(),
                            pos,
                        });
                    }
                } else if self.is_token(TokenKind::Comma, 0)? {
                    let (_, pos) = self.token().into_parts()?;
                    return Err(CalfErr {
                        message: "Not expecting a comma".into(),
                        pos,
                    });
                }

                if expect_param {
                    if self.is_token(TokenKind::Ident, 0)? {
                        let (param, _) = self.token().into_ident()?;
                        params.push(param);
                        expect_param = false;
                        expect_comma = true;
                    } else {
                        let (_, pos) = self.token().into_parts()?;
                        return Err(CalfErr {
                            message: "Expecting a parameter".into(),
                            pos,
                        });
                    }
                }
            }

            let body = Box::new(self.expression()?);

            return Ok(Expr::new(Syntagma::Lambda { params, body }, pos));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr<T>, CalfErr> {
        //TODO: parse list literals array and range
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
                let pos = self.token().unwrap().pos;
                return Err(CalfErr {
                    message: "Expected a closing parenthesis after expression".into(),
                    pos,
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
        //TODO: check the next token and see if we can provide a more specific error message
        // If we are here, something is badly formed
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
                let mut token = self.lexer.scan_token()?;
                // Skip None tokens (newlines and comments)
                while let Lexeme::None = token.lexeme {
                    token = self.lexer.scan_token()?;
                }
                // End Of File token, end getting tokens
                if let Lexeme::EOF = token.lexeme {
                    break;
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

    fn is_ident(&mut self, ident: &str, offset: usize) -> Result<bool, CalfErr> {
        if self.is_token(TokenKind::Ident, offset)? {
            if let Lexeme::Ident(lexeme_ident) = &self.tokens[offset].lexeme {
                return Ok(ident == lexeme_ident);
            }
        }
        Ok(false)
    }

    fn token(&mut self) -> Option<Token<T>> {
        self.tokens.pop_front()
    }

    pub fn is_end(&self) -> bool {
        self.tokens.len() == 0
    }
}
