use std::fmt;

pub struct Lexer;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<char> for Token {
    fn from(char: char) -> Token {
        match char {
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            _ => todo!("Unrecognised token, {}", char),
        }
    }
}

impl Lexer {
    // Lexical analysis:
    // Walk through the file contents, identifying the different parts of the JSON and mapping
    // those to the relevant lexeme
    pub fn run(file: &str) -> Result<Vec<Token>, String> {
        Lexer::analyse(Lexer::gather_tokens(file))
    }

    fn gather_tokens(file: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // Iteration 1: run through each line and parse blindly
        file.lines()
            .for_each(|line| line.chars().for_each(|char| tokens.push(Token::from(char))));

        return tokens;
    }

    fn analyse(
        tokens: Vec<Token>,
    ) -> Result<
        Vec<Token>,
        // TODO: We might want to return our own error type here
        String,
    > {
        // First, verify we have a valid opening brace
        let first_token = tokens.first();
        if first_token.is_none()
            || first_token.is_some() && first_token.unwrap() != &Token::OpenBrace
        {
            return Err(String::from("Missing idenitifier \"{\" at position 0"));
        }

        // At the end, we should validate the ending brace
        let last_token = tokens.last();
        if last_token.is_some() && last_token.unwrap() != &Token::CloseBrace {
            return Err(String::from("Missing identifier \"}\" at last position"));
        }

        Ok(tokens)
    }
}
