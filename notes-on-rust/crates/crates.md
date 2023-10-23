# Crates

A *crate* is the smallest unit that the Rust compilers works on at a time. This is also refereed to as that a crate is the basic unit of compilation in Rust. Crates comes in two versions *binary crates* and *library crates*.

## Binary Crate

Binary crates compiles into an executable program that can be run. Binary crates must have a function called `main`. This is a special function that will be the entry point of execution when running the executable.

## Library Crate

Library crates will not become an executable when compiled. This type of crate is instead intended to be a sub part of a project. Library crates will not have a `main` function.

In other programming languages a crate would be known as a library or a package.

## Crate Root

A crate can include multiple files organised in a tree structure that can have multiple source files. The source file that the Rust compiler starts from is known as the `crate root`. The root in a binary crate will be found in `src/main.rs`, and the root of a library crate is found in `src/lib.rs`.

## Package

Crates can be placed together to form a package. A package must include at least one crate. A package can have several binary crates but only one library crate.

## References

TODO
