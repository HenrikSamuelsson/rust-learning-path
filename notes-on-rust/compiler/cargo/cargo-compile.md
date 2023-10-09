# Compiling With Cargo

It is here explained how to create, compile and run a simple Rust program using Cargo.

Cargo is a package Manager for Rust. Cargo can do a lot besides compile Rust source code. In this section we will keep it basic and just briefly look into how to use cargo to create a simple but fully runnables Rust application.

## Compilation Steps

### Project Creation

In a terminal move to the folder where to create a new Rust project.

Example using PowerShell in Windows, `cd` is used to change to the desired directory and `ls` is used to check the current content:

```txt
PS C:\> cd C:\github\rust-learning-path\notes-on-rust\compiler\cargo
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo> ls


    Directory: C:\github\rust-learning-path\notes-on-rust\compiler\cargo


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2023-10-06     06:49            558 cargo-compile.md
```

Then, when being in the folder where we want to place the new project, use cargo to create a new project with the command `cargo new rust_test`. The `rust_test` part of the command is the chosen name that for this project.

Below is an example of running cargo to generate the project by the use of PowerShell on a computer running Windows:

```txt
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo> cargo new rust_test  
     Created binary (application) `rust_test` package
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo>
```

After cargo project creation there will be a new folder, in this case called `rust_test`, that in turn contains files and folders that make up a complete Rust project, ready for future development.

### Compilation

Cargo will when creating a new project also create the source code for a "Hello, world!" application. Before this application can be run it needs to be compiled to translate the source code to machine code that our computer can execute. Cargo can be used to compile with the command `cargo build` that shall be invoked in a terminal window when in the project folder of the project to be compiled.

```txt
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo\rust_test> cargo build
   Compiling rust_test v0.1.0 (C:\github\rust-learning-path\notes-on-rust\compiler\cargo\rust_test)  
    Finished dev [unoptimized + debuginfo] target(s) in 3.42s
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo\rust_test>
```

After running `cargo build` there will be additional content in the project and one of the artefacts will be a new runnable application.



## References

TODO add a reference to The Cargo Book.
