# Basic Compilation

It is here explained how to write and compile a simple Rust program. The way we compile here is a very basic workflow. We do this an exercise just to learn a little what happens under the hood when using higher level tools to compile Rust code into an executable.

## The Rust Compiler

Rust programs must be compiled before they can be executed. Compilation means that source code written by a human developer is translated into instructions that a computer can execute. The Rust compiler is installed on the computer when installing Rust. It is hence a requirement to have Rust installed to be able to do a compilation.

## Compilation Steps

### Project Folder

Shall create an empty folder dedicated just for content that will be created in this exercise.

Solution for this step in a powershell terminal on a computer running Windows:

1. The change directory command `cd` is used to move to the location where the new folder shall be created
2. The make directory command `mkdir` is used to create a new empty folder, the name chosen for the folder is rust-project
3. Finally change directory again with `cd` to move into the just created folder.

```txt
cd C:\github\rust-learning-path\notes-on-rust\compiler\rustc
mkdir rust-project
cd rust-project
```
