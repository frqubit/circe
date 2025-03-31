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

use cce_ast::*;

#[test]
fn test_parser_example_helloworld() {
    let mut parser = Parser::from(include_str!("./examples/hello.cce"));

    let expected_output = vec![
        ParseNode::Command(Command {
            components: vec![
                CommandComponent::Keyword("print".to_string()),
                CommandComponent::Literal("Hello, world!".to_string()),
                CommandComponent::Keyword("to".to_string()),
                CommandComponent::Keyword("the".to_string()),
                CommandComponent::Keyword("console".to_string()),
            ],
            modifiers: vec![],
        }),
        ParseNode::HowToStatement(HowToStatement {
            signature: vec![
                CommandComponent::Keyword("print".to_string()),
                CommandComponent::Keyword("a".to_string()),
                CommandComponent::Keyword("string".to_string()),
                CommandComponent::Keyword("to".to_string()),
                CommandComponent::Keyword("the".to_string()),
                CommandComponent::Keyword("console".to_string()),
            ],
            body: vec![Command {
                components: vec![
                    CommandComponent::Keyword("write".to_string()),
                    CommandComponent::Keyword("the".to_string()),
                    CommandComponent::Keyword("string".to_string()),
                    CommandComponent::Keyword("to".to_string()),
                    CommandComponent::Keyword("stdout".to_string()),
                ],
                modifiers: vec![vec![
                    CommandComponent::Keyword("add".to_string()),
                    CommandComponent::Keyword("a".to_string()),
                    CommandComponent::Keyword("newline".to_string()),
                ]],
            }],
        }),
        ParseNode::WhatIsStatement(WhatIsStatement {
            signature: vec![CommandComponent::Literal("stdout".to_string())],
            body: vec![WhatIsCommand::Command(Command {
                components: vec![
                    CommandComponent::Keyword("a".to_string()),
                    CommandComponent::Keyword("file".to_string()),
                    CommandComponent::Keyword("stream".to_string()),
                ],
                modifiers: vec![],
            })],
        }),
    ];

    let mut output: Vec<ParseNode> = Vec::new();
    while let Some(node) = parser.next().unwrap() {
        output.push(node)
    }

    assert_eq!(output, expected_output);
}
