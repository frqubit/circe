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

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramNode {
    Command(CommandNode),
    HowTo(HowToNode),
    WhatIs(WhatIsNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandNode {
    pub command: Vec<CommandComponent>,
    pub modifiers: Vec<Vec<CommandComponent>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HowToNode {
    pub signature: Vec<CommandComponent>,
    pub body: Vec<CommandNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhatIsNode {
    pub signature: Vec<CommandComponent>,
    pub body: Vec<WhatIsCommand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhatIsCommand {
    Command(CommandNode),
    Final(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandComponent {
    Literal(String),
    Keyword(String),
    Slot(String),
    BackRef(String),
}
