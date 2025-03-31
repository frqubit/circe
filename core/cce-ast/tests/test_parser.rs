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
fn test_parser_basic() {
    let mut parser = Parser::from("say hello world");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::Command(Command {
        components: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        modifiers: vec![],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_literal() {
    let mut parser = Parser::from("say 'hello world'");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::Command(Command {
        components: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Literal("hello world".to_string()),
        ],
        modifiers: vec![],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_modifier() {
    let mut parser = Parser::from("say hello world | say hello world");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::Command(Command {
        components: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        modifiers: vec![vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ]],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_modifier_multiple() {
    let mut parser = Parser::from("say hello world | say hello world | say hello world");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::Command(Command {
        components: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        modifiers: vec![
            vec![
                CommandComponent::Keyword("say".to_string()),
                CommandComponent::Keyword("hello".to_string()),
                CommandComponent::Keyword("world".to_string()),
            ],
            vec![
                CommandComponent::Keyword("say".to_string()),
                CommandComponent::Keyword("hello".to_string()),
                CommandComponent::Keyword("world".to_string()),
            ],
        ],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto() {
    let mut parser =
        Parser::from("howto say hello world?\n- say hello world\n| do not say goodbye");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
        signature: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        body: vec![Command {
            components: vec![
                CommandComponent::Keyword("say".to_string()),
                CommandComponent::Keyword("hello".to_string()),
                CommandComponent::Keyword("world".to_string()),
            ],
            modifiers: vec![vec![
                CommandComponent::Keyword("do".to_string()),
                CommandComponent::Keyword("not".to_string()),
                CommandComponent::Keyword("say".to_string()),
                CommandComponent::Keyword("goodbye".to_string()),
            ]],
        }],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto_multiple() {
    let mut parser = Parser::from(
        "howto say hello world?\n- say hello world\n| do not say goodbye\n- say hello world again",
    );

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
        signature: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        body: vec![
            Command {
                components: vec![
                    CommandComponent::Keyword("say".to_string()),
                    CommandComponent::Keyword("hello".to_string()),
                    CommandComponent::Keyword("world".to_string()),
                ],
                modifiers: vec![vec![
                    CommandComponent::Keyword("do".to_string()),
                    CommandComponent::Keyword("not".to_string()),
                    CommandComponent::Keyword("say".to_string()),
                    CommandComponent::Keyword("goodbye".to_string()),
                ]],
            },
            Command {
                components: vec![
                    CommandComponent::Keyword("say".to_string()),
                    CommandComponent::Keyword("hello".to_string()),
                    CommandComponent::Keyword("world".to_string()),
                    CommandComponent::Keyword("again".to_string()),
                ],
                modifiers: vec![],
            },
        ],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_whatis() {
    let mut parser = Parser::from("whatis the world?\n- a planet\n| in the universe");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::WhatIsStatement(WhatIsStatement {
        signature: vec![
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        body: vec![WhatIsCommand::Command(Command {
            components: vec![
                CommandComponent::Keyword("a".to_string()),
                CommandComponent::Keyword("planet".to_string()),
            ],
            modifiers: vec![vec![
                CommandComponent::Keyword("in".to_string()),
                CommandComponent::Keyword("the".to_string()),
                CommandComponent::Keyword("universe".to_string()),
            ]],
        })],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_whatis_multiple() {
    let mut parser = Parser::from(
        "whatis the world?\n- a planet\n| in the universe\n- a planet in the solar system",
    );

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::WhatIsStatement(WhatIsStatement {
        signature: vec![
            CommandComponent::Keyword("the".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        body: vec![
            WhatIsCommand::Command(Command {
                components: vec![
                    CommandComponent::Keyword("a".to_string()),
                    CommandComponent::Keyword("planet".to_string()),
                ],
                modifiers: vec![vec![
                    CommandComponent::Keyword("in".to_string()),
                    CommandComponent::Keyword("the".to_string()),
                    CommandComponent::Keyword("universe".to_string()),
                ]],
            }),
            WhatIsCommand::Command(Command {
                components: vec![
                    CommandComponent::Keyword("a".to_string()),
                    CommandComponent::Keyword("planet".to_string()),
                    CommandComponent::Keyword("in".to_string()),
                    CommandComponent::Keyword("the".to_string()),
                    CommandComponent::Keyword("solar".to_string()),
                    CommandComponent::Keyword("system".to_string()),
                ],
                modifiers: vec![],
            }),
        ],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_howto_multiple_nomod() {
    let mut parser =
        Parser::from("howto say hello world?\n- say hello world\n- say hello world again");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::HowToStatement(HowToStatement {
        signature: vec![
            CommandComponent::Keyword("say".to_string()),
            CommandComponent::Keyword("hello".to_string()),
            CommandComponent::Keyword("world".to_string()),
        ],
        body: vec![
            Command {
                components: vec![
                    CommandComponent::Keyword("say".to_string()),
                    CommandComponent::Keyword("hello".to_string()),
                    CommandComponent::Keyword("world".to_string()),
                ],
                modifiers: vec![],
            },
            Command {
                components: vec![
                    CommandComponent::Keyword("say".to_string()),
                    CommandComponent::Keyword("hello".to_string()),
                    CommandComponent::Keyword("world".to_string()),
                    CommandComponent::Keyword("again".to_string()),
                ],
                modifiers: vec![],
            },
        ],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
fn test_parser_slot() {
    let mut parser = Parser::from("read %hello.");

    let next_node: ParseNode = parser.next().unwrap().unwrap();
    let expected_node: ParseNode = ParseNode::Command(Command {
        components: vec![
            CommandComponent::Keyword("read".to_string()),
            CommandComponent::Slot("hello".to_string()),
        ],
        modifiers: vec![],
    });

    assert_eq!(next_node, expected_node);
}

#[test]
#[should_panic]
fn test_parser_openslot() {
    let mut parser = Parser::from("read %");

    parser.next().unwrap().unwrap();
}
