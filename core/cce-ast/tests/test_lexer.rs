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

use cce_ast::{Lexer, Token};

#[test]
fn test_lexer_basic() {
    let mut lexer = Lexer::from("howto hello world");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("world".to_string()));
}

#[test]
fn test_lexer_string() {
    let mut lexer = Lexer::from("howto 'hello world'");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Literal("hello world".to_string()));
}

#[test]
fn test_lexer_punct() {
    let mut lexer = Lexer::from("howto hello world\n- do it");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("world".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Newline);

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Punctuation('-'));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("do".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("it".to_string()));
}

#[test]
fn test_lexer_peek() {
    let mut lexer = Lexer::from("howto hello world");

    let next_token = lexer.peek().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    let next_token = lexer.peek().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("world".to_string()));
}

#[test]
fn test_lexer_final() {
    let mut lexer = Lexer::from("hello -$$ struct $Foo { $bar: u32 } $$.");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Punctuation('-'));

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(
        next_token,
        Token::FinalSequence(" struct $Foo { $bar: u32 } ".to_string())
    );
}

#[test]
#[should_panic]
fn test_lexer_open_string() {
    let mut lexer = Lexer::from("howto 'hello world");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Keyword("howto".to_string()));

    lexer.next().unwrap().unwrap();
}

#[test]
fn test_lexer_slot() {
    let mut lexer = Lexer::from("%hello");

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Percent);

    let next_token = lexer.next().unwrap().unwrap();
    assert_eq!(next_token, Token::Identifier("hello".to_string()));
}
