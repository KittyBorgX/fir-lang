# fir-lang

An elegant programming language

## The idea:

This is a language designed to be simple, having syntax somewhat similar to `go` and `python`, borrowing some object oriented programming concepts and techniques from `java` all while targetting `rust` and keeping an option of compilation through llvm (subject to change).

### Why am I making this?

Often, rust becomes really hard to write due to the strict type system and the borrow checker. This language aims at abstracting them from the user, letting the user focus on the task while handling other aspects during transpilation.

This way we get the gains of fast runtime performance of rust while also having simple syntax. Win-Win for all!

## Features

Along with the mentioned features, here are some more features which are notable and maybe implemented in the future.

- Simple Syntax.
- Memory safety guarantee (Transpiling to rust).
- Automatic parallelisation of codeblocks.
- Healing of errors.

## Syntax

Checkout [syntax.md](spec/syntax.md) for the syntax specification of the language.

Note : The syntax is subject to change since the language is in a really early stage.

