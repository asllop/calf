use alloc::{string::String, vec::Vec};
use core::{fmt::Debug, str::FromStr};
use logos::Logos;

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
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Assign,
    #[token("..")]
    Dots,
    #[token("[")]
    OpenClause,
    #[token("]")]
    ClosingClause,
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

#[derive(Debug)]
pub enum Lexeme<T> {
    Number(T),
    Ident(String),
    Other(TokenKind),
}

#[derive(Debug)]
pub struct Token<T> {
    pub lexeme: Lexeme<T>,
    pub row: usize,
    pub col: usize,
}

impl<T> Token<T> {
    pub fn new(lexeme: Lexeme<T>, row: usize, col: usize) -> Self {
        Self { lexeme, row, col }
    }
}

#[derive(Debug)]
/// Lexical analysis error.
pub struct LexError {
    /// Error message.
    pub message: String,
    /// Line where the error was found.
    pub row: usize,
    /// Position in the line where the error was found.
    pub col: usize,
}

pub fn scan<T>(code: &str) -> Result<Vec<Token<T>>, LexError>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut tokens = vec![];
    let mut row = 0;
    let mut offset = 0;
    let mut col;

    for (lexeme, pos) in TokenKind::lexer(code).spanned() {
        let fragment = &code[pos.start..pos.end];
        col = pos.start - offset;

        if let Ok(lexeme) = lexeme {
            match lexeme {
                TokenKind::Comment => {}
                TokenKind::EOL => {
                    row += 1;
                    offset = pos.end;
                }
                TokenKind::Int | TokenKind::Float => {
                    let n = str::parse::<T>(fragment).unwrap();
                    tokens.push(Token::new(Lexeme::Number(n), row, col))
                }
                TokenKind::Ident => {
                    tokens.push(Token::new(Lexeme::Ident(fragment.into()), row, col))
                }
                _ => tokens.push(Token::new(Lexeme::Other(lexeme), row, col)),
            }
        } else {
            return Err(LexError {
                message: format!("Unrecognized lexeme: '{}'", fragment),
                row,
                col,
            });
        }
    }
    Ok(tokens)
}
