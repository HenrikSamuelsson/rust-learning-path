# Compiling With Cargo

It is here explained how to write and compile a simple Rust program using Cargo.

Cargo is a package Manager for Rust. Cargo can do a lot besides compile Rust source code. In this section we will keep it basic and just look into how to use cargo to compile a very simple Rust project.

## Compilation Steps

### Project Folder Creation

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

Then when being in the folder where we want the new project we use cargo to create a new project here, we choose the name `rust_test` for our project:

```txt
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo> cargo new rust_test  
     Created binary (application) `rust_test` package
PS C:\github\rust-learning-path\notes-on-rust\compiler\cargo>
```

## References

TODO add a reference to The Cargo Book.
