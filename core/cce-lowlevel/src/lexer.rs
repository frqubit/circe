/*

Copyright (C) 2023 Carlos Kieliszewski

This file is part of the Circe Project.

Circe is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

Circe is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Circe. If not, see <https://www.gnu.org/licenses/>. 

*/


use cce_stream::{InputStream, InputStreamError};

use thiserror::Error;


pub struct Lexer {
  pub(crate) stream: InputStream,
  pub(crate) peeked: Option<Token>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Variable(String),
  Identifier(String),
  Literal(String),
  Punctuation(char),
  HighLevelSequence(String)
}

#[derive(Error, Debug)]
pub enum LexerError {
  #[error("{0}")]
  InputStreamError(#[from] InputStreamError),
  #[error("Unexpected character: {0}")]
  UnexpectedCharacter(char),
  #[error("Unexpected end of input")]
  UnexpectedEndOfInput
}


impl Lexer {
  pub fn new(stream: InputStream) -> Self {
    Self {
      stream,
      peeked: None
    }
  }

  fn create_ident(&mut self) -> Result<String, LexerError> {
    let mut ident = String::new();
    let mut c: char = self.stream.peek()?.ok_or(LexerError::UnexpectedEndOfInput)?;

    while c.is_alphanumeric() {
      ident.push(c);
      self.stream.next()?;
      c = match self.stream.peek()? {
        Some(c) => c,
        None => break
      };
    }

    Ok(ident)
  }

  fn create_literal(&mut self) -> Result<String, LexerError> {
    let mut literal = String::new();
    let mut c: char = self.stream.peek()?.ok_or(LexerError::UnexpectedEndOfInput)?;

    while c.is_numeric() {
      literal.push(c);
      self.stream.next()?;
      c = match self.stream.peek()? {
        Some(c) => c,
        None => break
      };
    }

    Ok(literal)
  }

  fn create_high_level_sequence(&mut self) -> Result<Token, LexerError> {
    let mut sequence = String::new();
    let mut c: char = self.stream.peek()?.ok_or(LexerError::UnexpectedEndOfInput)?;

    while c != '>' {
      sequence.push(c);
      self.stream.next()?;
      c = self.stream.peek()?.ok_or(LexerError::UnexpectedEndOfInput)?;
    }

    self.stream.next()?;
    Ok(Token::HighLevelSequence(sequence))
  }

  pub fn next(&mut self) -> Result<Option<Token>, LexerError> {
    if self.peeked.is_some() {
      let peeked = self.peeked.clone();
      self.peeked = None;
      return Ok(peeked);
    }

    let mut c: char = match self.stream.peek()? {
      Some(c) => c,
      None => return Ok(None)
    };

    while c.is_whitespace() {
      self.stream.next()?;
      c = match self.stream.peek()? {
        Some(c) => c,
        None => return Ok(None)
      }
    };

    match c {
      '@' => {
        self.stream.next()?;
        Ok(Some(Token::Variable(self.create_ident()?)))
      },
      '0'..='9' => {
        Ok(Some(Token::Literal(self.create_literal()?)))
      },
      'A'..='Z' => {
        Ok(Some(Token::Identifier(self.create_ident()?)))
      },
      '=' => {
        self.stream.next()?;
        Ok(Some(Token::Punctuation('=')))
      },
      '<' => {
        self.stream.next()?;
        Ok(Some(self.create_high_level_sequence()?))
      },
      _ => Err(LexerError::UnexpectedCharacter(c))
    }
  }

  pub fn peek(&mut self) -> Result<Option<Token>, LexerError> {
    if self.peeked.is_none() {
      self.peeked = self.next()?;
    }

    Ok(self.peeked.clone())
  }
}