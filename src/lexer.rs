use alloc::string::String;
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
    Empty,
    EOL,
    EOF,
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

#[derive(Debug, Clone)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

pub fn scan_token<T>(code: &str, prev_pos: Pos) -> Result<(Token<T>, &str, Pos), LexError>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    if let Some((lexeme, lex_offs)) = TokenKind::lexer(code).spanned().next() {
        let fragment = &code[lex_offs.start..lex_offs.end];
        let code_rest = &code[lex_offs.end..];

        let col = lex_offs.start + prev_pos.col;
        let mut next_pos = Pos::new(prev_pos.row, col);

        if let Ok(lexeme) = lexeme {
            
            match lexeme {
                TokenKind::Comment => {
                    let token = Token::new(Lexeme::Empty, next_pos.clone());
                    next_pos.col += lex_offs.end - lex_offs.start;
                    Ok((token, code_rest, next_pos))
                }
                TokenKind::EOL => {
                    let token = Token::new(Lexeme::EOL, next_pos.clone());
                    next_pos.row += 1;
                    next_pos.col = 0;
                    Ok((token, code_rest, next_pos))
                }
                TokenKind::Int | TokenKind::Float => {
                    let n = str::parse::<T>(fragment).unwrap();
                    let token = Token::new(Lexeme::Number(n), next_pos.clone());
                    next_pos.col += lex_offs.end - lex_offs.start;
                    Ok((token, code_rest, next_pos))
                }
                TokenKind::Ident => {
                    let token = Token::new(Lexeme::Ident(fragment.into()), next_pos.clone());
                    next_pos.col += lex_offs.end - lex_offs.start;
                    Ok((token, code_rest, next_pos))
                }
                _ => {
                    let token = Token::new(Lexeme::Other(lexeme), next_pos.clone());
                    next_pos.col += lex_offs.end - lex_offs.start;
                    Ok((token, code_rest, next_pos))
                }
            }
        } else {
            return Err(LexError {
                message: format!("Unrecognized lexeme: '{}'", fragment),
                row: next_pos.row,
                col: next_pos.col,
            });
        }
    } else {
        // EOF
        let token = Token::new(Lexeme::EOF, prev_pos.clone());
        Ok((token, "", prev_pos))
    }
}
