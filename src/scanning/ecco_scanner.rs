use super::ecco_token::{Token, TokenType};
use crate::util::ecco_error;
use anyhow::Result;
use std::collections::VecDeque;
use std::fs;

pub struct Scanner {
    filename: String,
    file_contents: VecDeque<char>,
}

impl Scanner {
    /// Constrtucts a scanner for Tokens.
    /// Args: filename: String
    pub fn new(filename: String) -> Result<Scanner> {
        let contents: VecDeque<char> = fs::read_to_string(&filename)?.chars().collect();
        Ok(Scanner {
            filename: filename,
            file_contents: contents,
        })
    }

    /// Scan for the next token and return? it. Option<Token> contains None if EOF has been reached.
    fn scan(&mut self) -> Result<Option<Token>> {
        let mut current_char = self.file_contents.pop_front();
        while {
            match current_char {
                None => return Ok(None), // There is no more token (i.e. the deque was empty when pop() was called)
                Some(item) => item.is_whitespace(), // this is the statement that will utlimately be evaluated by while
            }
        } {
            current_char = self.file_contents.pop_front(); // pop again if we found a whitespace
        }
        let current_char = current_char.unwrap(); // we now know we have a valid token, get the bare char ready.
        let mut current_string = current_char.to_string(); // we now know we have a valid token, get it back into a string

        // Now, check if it's a digit and look for more digits

        if current_char.is_ascii_digit() {
            // keep popping all the digits and append to the string
            while match self.file_contents.front() {
                Some(item) => item.is_ascii_digit(), // this will be evaluated by while
                None => false,                       // there are no more items
            } {
                current_string.push(self.file_contents.pop_front().unwrap());
            }
            let tokentype = TokenType::INTEGER_LITERAL;
            let value: i32 = match current_string.parse::<i32>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(ecco_error::EccoSyntaxError::new(format!(
                        "Failed to parse integer token: {}",
                        current_string
                    ))
                    .into())
                }
            };
            return Ok(Some(Token::new(Some(tokentype), Some(value))));
        }

        // Otherwise, handle a normal token
        let tokentype: TokenType = match &current_string[..] {
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "*" => TokenType::STAR,
            "/" => TokenType::SLASH,
            _ => {
                return Err(ecco_error::EccoSyntaxError::new(format!(
                    "Unrecognized token: {}",
                    current_string
                ))
                .into())
            }
        };
        Ok(Some(Token::new(Some(tokentype), None)))
    }
    /// Scans a file and prints its tokens
    pub fn scan_file(&mut self) -> Result<()> {
        while let Some(token) = self.scan()? {
            println!("{}", token);
        }
        Ok(())
    }
}
