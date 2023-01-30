use anyhow::{Context, Result};
use ecco_token::{Token, TokenType};
use std::collections;
use std::fs;

struct Scanner {
    filename: String,
    file_contents: collections::VecDeque<char>,
}

impl Scanner {
    /// Constrtucts a scanner for Tokens.
    /// Args: filename: String
    pub fn new(filename: String) -> Result<Scanner> {
        let contents: VecDeque<char> = fs::read_to_string(filename)?.chars().collect();
        Ok(Scanner {
            filename: filename,
            file_contents: contents,
        })
    }

    fn scan(&mut self) -> Result<Option<Token>> {
        
    }
}
