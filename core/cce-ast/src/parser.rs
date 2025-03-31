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

use crate::lexer::{Lexer, LexerError, Token};
use circelang_hash::CirceHash;

use thiserror::Error;

pub struct Parser<'s> {
    pub(crate) lexer: Lexer<'s>,
    pub(crate) peeked: Option<ParseNode>,
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub enum ParseNode {
    Command(Command),
    HowToStatement(HowToStatement),
    WhatIsStatement(WhatIsStatement),
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub struct Command {
    pub components: Vec<CommandComponent>,
    pub modifiers: Vec<Vec<CommandComponent>>,
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub enum CommandComponent {
    Literal(String),
    Keyword(String),
    Slot(String),
    BackRef(String),
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub struct HowToStatement {
    pub signature: Vec<CommandComponent>,
    pub body: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub enum WhatIsCommand {
    Command(Command),
    Final(String),
}

#[derive(Debug, Clone, PartialEq, CirceHash)]
pub struct WhatIsStatement {
    pub signature: Vec<CommandComponent>,
    pub body: Vec<WhatIsCommand>,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("{0}")]
    LexerError(#[from] LexerError),
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl<'s> Parser<'s> {
    pub fn new(lexer: Lexer<'s>) -> Parser {
        Parser {
            lexer,
            peeked: None,
        }
    }

    fn parse_vec_command_component(&mut self) -> Result<Vec<CommandComponent>, ParserError> {
        let mut components: Vec<CommandComponent> = Vec::new();

        let mut tok: Option<Token> = self.lexer.peek()?;

        while let Some(token) = tok.clone() {
            match token {
                Token::Identifier(ident) => {
                    components.push(CommandComponent::Keyword(ident));
                }
                Token::Keyword(kw) => {
                    components.push(CommandComponent::Keyword(kw));
                }
                Token::Literal(lit) => {
                    components.push(CommandComponent::Literal(lit));
                }
                Token::Percent => {
                    self.lexer.next()?;
                    tok = self.lexer.peek()?;

                    if let Some(Token::Identifier(ident)) = tok {
                        components.push(CommandComponent::Slot(ident));
                    } else {
                        return Err(ParserError::SyntaxError("Expected identifier".to_string()));
                    }
                }
                Token::Ampersand => {
                    self.lexer.next()?;
                    tok = self.lexer.peek()?;

                    if let Some(Token::Identifier(ident)) = tok {
                        components.push(CommandComponent::BackRef(ident));
                    } else {
                        return Err(ParserError::SyntaxError("Expected identifier".to_string()));
                    }
                }
                Token::Punctuation(_) => {
                    break;
                }
                Token::FinalSequence(_) => {
                    return Err(ParserError::SyntaxError(
                        "Final sequences are not allowed here".to_string(),
                    ));
                }
                Token::Newline => {
                    break;
                }
                Token::Question => {
                    break;
                }
                Token::Dot => {
                    break;
                }
            }

            self.lexer.next()?;
            tok = self.lexer.peek()?;
        }

        Ok(components)
    }

    fn parse_command(&mut self) -> Result<Command, ParserError> {
        let components: Vec<CommandComponent> = self.parse_vec_command_component()?;
        let mut modifiers: Vec<Vec<CommandComponent>> = Vec::new();

        let mut tok: Option<Token> = self.lexer.peek()?;

        while let Some(token) = tok.clone() {
            match token {
                Token::Punctuation(punc) => match punc {
                    '|' => {
                        self.lexer.next()?;
                        modifiers.push(self.parse_vec_command_component()?);
                        tok = self.lexer.peek()?;
                    }
                    '-' => {
                        break;
                    }
                    _ => {
                        return Err(ParserError::SyntaxError("Expected '|'".to_string()));
                    }
                },
                Token::Dot => {
                    self.lexer.next()?;
                    break;
                }
                Token::Newline => {
                    self.lexer.next()?;
                    tok = self.lexer.peek()?;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(Command {
            components,
            modifiers,
        })
    }

    fn parse_whatis_command(&mut self) -> Result<WhatIsCommand, ParserError> {
        let tok: Option<Token> = self.lexer.peek()?;

        match tok {
            Some(Token::FinalSequence(seq)) => {
                self.lexer.next()?;

                Ok(WhatIsCommand::Final(seq))
            }
            _ => Ok(WhatIsCommand::Command(self.parse_command()?)),
        }
    }

    fn parse_howto_statement(&mut self) -> Result<HowToStatement, ParserError> {
        let signature: Vec<CommandComponent> = self.parse_vec_command_component()?;

        if self.lexer.peek()? != Some(Token::Question) {
            return Err(ParserError::SyntaxError("Expected '?'".to_string()));
        }

        self.lexer.next()?;

        if self.lexer.peek()? != Some(Token::Newline) {
            return Err(ParserError::SyntaxError("Expected newline".to_string()));
        }

        self.lexer.next()?;

        let mut body: Vec<Command> = Vec::new();

        let mut tok: Option<Token> = self.lexer.peek()?;

        match tok {
            Some(Token::Punctuation(punc)) => {
                if punc == '-' {
                    self.lexer.next()?;
                } else {
                    return Err(ParserError::SyntaxError("Expected '-'".to_string()));
                }
            }
            _ => {
                return Err(ParserError::SyntaxError("Expected '-'".to_string()));
            }
        };

        loop {
            let cmd: Command = self.parse_command()?;
            body.push(cmd);

            tok = self.lexer.peek()?;
            match tok {
                Some(Token::Punctuation('-')) => {
                    self.lexer.next()?;
                }
                Some(Token::Dot) => {
                    self.lexer.next()?;
                    break;
                }
                Some(Token::Punctuation(_)) => {
                    return Err(ParserError::SyntaxError("Expected '-' or '.'".to_string()));
                }
                _ => break,
            }
        }

        Ok(HowToStatement { signature, body })
    }

    fn parse_whatis_statement(&mut self) -> Result<WhatIsStatement, ParserError> {
        let signature: Vec<CommandComponent> = self.parse_vec_command_component()?;

        if self.lexer.peek()? != Some(Token::Question) {
            return Err(ParserError::SyntaxError("Expected '?'".to_string()));
        }

        self.lexer.next()?;

        if self.lexer.peek()? != Some(Token::Newline) {
            return Err(ParserError::SyntaxError("Expected newline".to_string()));
        }

        self.lexer.next()?;

        let mut body: Vec<WhatIsCommand> = Vec::new();

        let mut tok: Option<Token> = self.lexer.peek()?;

        match tok {
            Some(Token::Punctuation(punc)) => {
                if punc == '-' {
                    self.lexer.next()?;
                } else {
                    return Err(ParserError::SyntaxError("Expected '-'".to_string()));
                }
            }
            _ => {
                return Err(ParserError::SyntaxError("Expected '-'".to_string()));
            }
        };

        loop {
            let cmd: WhatIsCommand = self.parse_whatis_command()?;
            body.push(cmd);

            tok = self.lexer.peek()?;
            match tok {
                Some(Token::Punctuation('-')) => {
                    self.lexer.next()?;
                }
                Some(Token::Newline) => {
                    self.lexer.next()?;

                    match self.lexer.peek()? {
                        Some(Token::Newline) => {
                            self.lexer.next()?;
                            break;
                        }
                        Some(Token::Punctuation('-')) => {
                            self.lexer.next()?;
                        }
                        None => {
                            break;
                        }
                        _ => {
                            return Err(ParserError::SyntaxError(
                                "Expected newline or '-'".to_string(),
                            ));
                        }
                    }
                }
                Some(Token::Punctuation(_)) => {
                    return Err(ParserError::SyntaxError("Expected '-'".to_string()));
                }
                _ => break,
            }
        }

        Ok(WhatIsStatement { signature, body })
    }

    // TODO: Move this to an iterator
    pub fn next(&mut self) -> Result<Option<ParseNode>, ParserError> {
        if self.peeked.is_some() {
            let peeked: Option<ParseNode> = self.peeked.clone();
            self.peeked = None;
            return Ok(peeked);
        }

        let mut token: Token = match self.lexer.peek()? {
            Some(tok) => tok,
            None => {
                return Ok(None);
            }
        };

        while token == Token::Newline {
            self.lexer.next()?;
            token = match self.lexer.peek()? {
                Some(tok) => tok,
                None => {
                    return Ok(None);
                }
            };
        }

        match token {
            Token::Keyword(kw) => match kw.as_str() {
                "howto" => {
                    self.lexer.next()?;
                    let howto: HowToStatement = self.parse_howto_statement()?;
                    Ok(Some(ParseNode::HowToStatement(howto)))
                }
                "whatis" => {
                    self.lexer.next()?;
                    let whatis: WhatIsStatement = self.parse_whatis_statement()?;
                    Ok(Some(ParseNode::WhatIsStatement(whatis)))
                }
                _ => Err(ParserError::InternalError("Unexpected keyword".to_string())),
            },
            Token::Identifier(_) => Ok(Some(ParseNode::Command(self.parse_command()?))),
            _ => Err(ParserError::InternalError("Unexpected token".to_string())),
        }
    }

    pub fn peek(&mut self) -> Result<Option<ParseNode>, ParserError> {
        if self.peeked.is_none() {
            self.peeked = self.next()?;
        }

        Ok(self.peeked.clone())
    }
}

impl<'s> From<&'s str> for Parser<'s> {
    fn from(data: &'s str) -> Self {
        Parser::new(Lexer::from(data))
    }
}
