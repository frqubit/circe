# Circe - Concepts

Circe is built on the idea that code/algorithms are designed in blocks. The CIE is designed to take this idea to its absolute extremes. It essentially functions as a super advanced templating engine.

At its core, Circe is organized around three main statement types: commands, howto, and whatis.

## Commands

Commands are what you traditionally think of as computer instructions. Any specific action that a system needs to do or create are handled by commands. They can perform actions on their own or can generate objects.

## `howto`

howto statements define instructions or specific additions for commands. Any command needs a howto statement corresponding to it to work properly.

Commands will choose the most common howto according to its specifiers.

## `whatis`

whatis statements are composed of additional instructions for generation or a final output. All instructions must eventually lead to a final howto or whatis, which outputs generated content according to its template.
