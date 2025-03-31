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

use crate::nodes::*;
use cce_ast as ast;

pub fn convert(program: Vec<ast::ParseNode>) -> Vec<ProgramNode> {
    program
        .into_iter()
        .map(|node| match node {
            ast::ParseNode::Command(command) => ProgramNode::Command(convert_command(command)),
            ast::ParseNode::HowToStatement(howto) => ProgramNode::HowTo(convert_howto(howto)),
            ast::ParseNode::WhatIsStatement(whatis) => ProgramNode::WhatIs(convert_whatis(whatis)),
        })
        .collect()
}

fn convert_command(command: ast::Command) -> CommandNode {
    CommandNode {
        command: command
            .components
            .into_iter()
            .map(convert_command_component)
            .collect(),
        modifiers: command
            .modifiers
            .into_iter()
            .map(|modifier| {
                modifier
                    .into_iter()
                    .map(convert_command_component)
                    .collect()
            })
            .collect(),
    }
}

fn convert_command_component(component: ast::CommandComponent) -> CommandComponent {
    match component {
        ast::CommandComponent::Literal(literal) => CommandComponent::Literal(literal),
        ast::CommandComponent::Keyword(keyword) => CommandComponent::Keyword(keyword),
        ast::CommandComponent::Slot(slot) => CommandComponent::Slot(slot),
        ast::CommandComponent::BackRef(backref) => CommandComponent::BackRef(backref),
    }
}

fn convert_howto(howto: ast::HowToStatement) -> HowToNode {
    HowToNode {
        signature: howto
            .signature
            .into_iter()
            .map(convert_command_component)
            .collect(),
        body: howto.body.into_iter().map(convert_command).collect(),
    }
}

fn convert_whatis_command(command: ast::WhatIsCommand) -> WhatIsCommand {
    match command {
        ast::WhatIsCommand::Command(command) => WhatIsCommand::Command(convert_command(command)),
        ast::WhatIsCommand::Final(lowlevel) => WhatIsCommand::Final(lowlevel),
    }
}

fn convert_whatis(whatis: ast::WhatIsStatement) -> WhatIsNode {
    WhatIsNode {
        signature: whatis
            .signature
            .into_iter()
            .map(convert_command_component)
            .collect(),
        body: whatis
            .body
            .into_iter()
            .map(convert_whatis_command)
            .collect(),
    }
}
