use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub file: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a str) -> Self {
        Parser { file }
    }

    pub fn parse(&self) -> Result<(), String> {
        Lexer::run(&self.file)?;
        Ok(())
    }
}
