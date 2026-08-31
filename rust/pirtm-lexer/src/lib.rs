use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"//.*")]
pub enum Token {
    #[token("let")] Let,
    #[token("mut")] Mut,
    #[token("fn")] Fn,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("return")] Return,
    #[token("ensemble")] Ensemble,
    #[token("use")] Use,
    #[token("extern")] Extern,

    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("=")] Equal,
    #[token("==")] EqEq,
    #[token("!=")] Neq,
    #[token("&&")] AndAnd,
    #[token("||")] OrOr,
    #[token("!")] Bang,
    #[token("<")] Lt,
    #[token(">")] Gt,
    #[token("<=")] Le,
    #[token(">=")] Ge,

    #[token("(")] LPar,
    #[token(")")] RPar,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(",")] Comma,
    #[token(";")] Semicolon,
    #[token(":")] Colon,
    #[token("::")] ColonColon,
    #[token(".")] Dot,
    #[token("with")] With,
    #[token("as")] As,
    #[token("=>")] FatArrow,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<u64>().unwrap())]
    Integer(u64),

    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLit(String),

    #[regex(r#"'([^'\\]|\\.)'"#, |lex| {
        let s = lex.slice();
        s.chars().nth(1).unwrap()
    })]
    CharLit(char),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(source);
    while let Some(tok) = lexer.next() {
        if let Ok(t) = tok {
            tokens.push(t);
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lex() {
        let binding = std::fs::read_to_string("../calculator.pirtm").unwrap();
        let mut lex = Token::lexer(binding.as_str());
        while let Some(tok) = lex.next() {
            println!("{:?}", tok);
        }
    }
}
