use logos::Logos;


#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t]+")]
/// Token types.
pub enum TokenKind {
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

    // Comment
    #[token("//")]
    Comment,

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

pub enum Lexeme<T> {
    Number(T),
    Ident(String),
    Other(TokenKind),
}

pub struct Token<T> {
    pub lexeme: Lexeme<T>,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
/// Lexical analysis error.
pub struct LexError {
    /// Error message.
    pub message: String,
    /// Position in the line where the error was found.
    pub position: usize,
}

pub fn scan(code: &str) -> Result<(), LexError> {
    for (lexeme, pos) in TokenKind::lexer(code).spanned() {
        let fragment = &code[pos.start..pos.end];
        let lex_pos = pos.start;
        if let Ok(lexeme) = lexeme {
            if let TokenKind::Ident | TokenKind::Int | TokenKind::Float = lexeme {
                println!("{:?}({})", lexeme, fragment);
            }
            else {
                println!("{:?}", lexeme);
            }
        }
        else {
            return Err(LexError {
                message: format!("Unrecognized lexeme: '{}'", fragment),
                position: lex_pos,
            });
        }
    }
    Ok(())
}