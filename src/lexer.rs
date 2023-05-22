use crate::common::{CalfErr, Pos};
use alloc::string::String;
use core::{fmt::Debug, str::FromStr};
use logos::Logos;

//TODO: Add tokens: NAN, +INF, -INF

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t]+")]
/// Token types.
pub enum TokenKind {
    // Comment
    #[regex("//.*")]
    Comment,

    // End Of Line
    #[token("\n")]
    EOL,

    // Single and double char
    #[token("(")]
    OpenParenth,
    #[token(")")]
    ClosingParenth,
    #[token("[")]
    OpenClause,
    #[token("]")]
    ClosingClause,
    #[token("{")]
    OpenCurly,
    #[token("}")]
    ClosingCurly,
    #[token(",")]
    Comma,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    LesserThan,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GtEqual,
    #[token("<=")]
    LtEqual,
    #[token("&")]
    And,
    #[token("&&")]
    TwoAnds,
    #[token("|")]
    Or,
    #[token("||")]
    TwoOrs,
    #[token("!")]
    Not,
    #[token("==")]
    TwoEquals,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Assign,
    #[token(".")]
    Dot,
    #[token("..")]
    TwoDots,
    #[token("#")]
    Sharp,

    // Literals
    #[regex("-?[0-9]+")]
    Int,
    //TODO: scientific notation: -9.09E-3, 9.09E+3
    #[regex(r#"-?[0-9]+\.[0-9]+"#)]
    Float,

    // Multichar
    #[regex(r#"[\p{Alphabetic}_]([\p{Alphabetic}_0-9]+)?"#)]
    Ident,
}

#[derive(Debug, PartialEq)]
pub enum Lexeme<T> {
    Number(T),
    Ident(String),
    Particle(TokenKind),
    EOF,
    None,
}

#[derive(Debug)]
pub struct Token<T> {
    pub lexeme: Lexeme<T>,
    pub pos: Pos,
}

impl<T> Token<T> {
    pub fn new(lexeme: Lexeme<T>, pos: Pos) -> Self {
        Self { lexeme, pos }
    }

    pub fn particle(&self) -> Result<TokenKind, CalfErr> {
        if let Lexeme::Particle(t) = self.lexeme {
            Ok(t)
        } else {
            Err(CalfErr {
                message: "Expected a particle".into(),
                pos: self.pos.clone(),
            })
        }
    }
}

pub struct Lexer<'a> {
    current_code: &'a str,
    last_pos: Pos,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            current_code: code,
            last_pos: Pos::new(0, 0),
        }
    }

    pub fn pos(&self) -> Pos {
        self.last_pos.clone()
    }

    pub fn scan_token<T>(&mut self) -> Result<Token<T>, CalfErr>
    where
        T: FromStr + Debug + PartialEq,
        <T as FromStr>::Err: Debug,
    {
        if let Some((lexeme, lex_offs)) = TokenKind::lexer(self.current_code).spanned().next() {
            let fragment = &self.current_code[lex_offs.start..lex_offs.end];
            self.current_code = &self.current_code[lex_offs.end..];

            let col = lex_offs.start + self.last_pos.col;
            let mut next_pos = Pos::new(self.last_pos.row, col);

            if let Ok(lexeme) = lexeme {
                match lexeme {
                    TokenKind::Comment => {
                        let token = Token::new(Lexeme::None, next_pos.clone());
                        next_pos.col += lex_offs.end - lex_offs.start;
                        self.last_pos = next_pos;
                        Ok(token)
                    }
                    TokenKind::EOL => {
                        let token = Token::new(Lexeme::None, next_pos.clone());
                        next_pos.row += 1;
                        next_pos.col = 0;
                        self.last_pos = next_pos;
                        Ok(token)
                    }
                    TokenKind::Int | TokenKind::Float => match str::parse::<T>(fragment) {
                        Ok(n) => {
                            let token = Token::new(Lexeme::Number(n), next_pos.clone());
                            next_pos.col += lex_offs.end - lex_offs.start;
                            self.last_pos = next_pos;
                            Ok(token)
                        }
                        Err(err) => Err(CalfErr {
                            message: format!("{:?}", err),
                            pos: next_pos,
                        }),
                    },
                    TokenKind::Ident => {
                        let token = Token::new(Lexeme::Ident(fragment.into()), next_pos.clone());
                        next_pos.col += lex_offs.end - lex_offs.start;
                        self.last_pos = next_pos;
                        Ok(token)
                    }
                    _ => {
                        let token = Token::new(Lexeme::Particle(lexeme), next_pos.clone());
                        next_pos.col += lex_offs.end - lex_offs.start;
                        self.last_pos = next_pos;
                        Ok(token)
                    }
                }
            } else {
                return Err(CalfErr {
                    message: format!("Unrecognized lexeme: '{}'", fragment),
                    pos: next_pos,
                });
            }
        } else {
            // EOF
            let token = Token::new(Lexeme::EOF, self.last_pos.clone());
            Ok(token)
        }
    }
}
