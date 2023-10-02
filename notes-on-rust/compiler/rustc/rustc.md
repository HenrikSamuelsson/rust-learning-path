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

There are of course multiple other ways to create an empty folder on a computer, such as using a file explorer. The exact way used to create the folder does not matter as long as the end result is an new empty folder with a known location.

### Create Rust Source Code

The Rust Compiler shall be provided a piece of source code to compile. We write the following Rust code in a text editor.

```rust
fn main() {
    println!("Hello, world!");
}
```

Save the code in a file called `main.rs` inside the folder created in the previous step.

There should now be a single file in the folder. This can be verified by listing the folder content. Example of this in a power shell terminal:

```txt
PS C:\github\rust-learning-path\notes-on-rust\compiler\rustc\rust-project> ls


    Directory: C:\github\rust-learning-path\notes-on-rust\compiler\rustc\rust-project


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2023-10-02     06:36             48 main.rs

```