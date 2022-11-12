# Daily Log November 2022

Notes on the daily steps taken during completion of this Rust Learning Path project.

## 2022-11-06

### Class Central - 10 Best Rust Courses to Take in 2022

Found a list of Rust courses at [Class Central - "10 Best Rust Courses to Take in 2022"](https://www.classcentral.com/report/best-rust-courses/). A bit busy at the moment but will try to find the time to take some of these courses.

## 2022-11-07

### Intro to Rust Functions

Read about functions, noted that there does not seem to be any need for forward declaration of functions, at least not in the same file. Also learned that the last expression in a function will be returned and this line shall not end with a semicolon since it would then be a statement and not an expression. Me personally does not like that it is done this way when being used to that there should be a return keyword to return something from a function.

### The Rust Book Experiment

Found out about an alternative version of the on-line Rust Book that comes with integrated quizzes, [Rust Book Experiment](https://rust-book.cs.brown.edu/experiment-intro.html). Will read this one since doing the quizzes should help to make the learnings stick.

## 2022-11-08

### Rust Updated

Read in online version of [The Rust Programming Language](https://rust-book.cs.brown.edu/title-page.html) book that I should now use Rust version 1.62.

Checked my installed rust version:

```txt
$ rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
```

So now behind the recommended version. Used rustup to update:

``` txt
$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2022-11-03, rust version 1.65.0 (897e37553 2022-11-02)
...
...
...
  stable-x86_64-pc-windows-msvc updated - rustc 1.65.0 (897e37553 2022-11-02) (from rustc 1.61.0 (fe5b13d68 2022-05-18))

info: cleaning up downloads & tmp directories
```

Turned out that there was now a 1.65 version so the book is behind. Choose to go with version 1.65 anyway figuring that the book will catch up sooner or later anyway.

### About Rust Installation

There is a tool called rustup used to manage rust versions and associated tools. THis is not to be confused with the tool called rustc that is the Rust compiler.

Example usages:

```txt
$  rustup --version
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.65.0 (897e37553 2022-11-02)`
```

```txt
$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: checking for self-updates

  stable-x86_64-pc-windows-msvc unchanged - rustc 1.65.0 (897e37553 2022-11-02)

info: cleaning up downloads & tmp directories
```

## 2022-11-09

Received delivery of a development board called [STM32F303 Discovery kit](https://www.st.com/en/evaluation-tools/stm32f3discovery.html). Will use this to try embedded Rust development, but will first need to learn Rust in general better. Anyway the board have an Arm M4 based MCU, an acceleration sensor, LED's and a user push button, and a USB debug port. Should be useful as it stands for a wide variety of experiments with embedded Rust.

## 2022-11-10

Reading up on the Rust build system called Cargo. Checked what version of Cargo that I have:

```txt
$ cargo --version
cargo 1.65.0 (4bc8f24d3 2022-10-20)
```

Then took cargo for a test-run by creating a dummy project, building, and running:

```txt
$ cargo new hello
     Created binary (application) `hello` package

$ cd hello

$ cargo build
   Compiling hello v0.1.0 (H:\temporary\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 10.50s

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 2.13s
     Running `target\debug\hello.exe`
Hello, world!
```

Have done this before in powershell and now tested with Windows cmd prompt, this worked equally well.

## 2022-11-11

### The Rust **const** Keyword

Learned about the **const** keyword that can be used to create constant values that the entire program can always see and use. Could for example use this keyword to define pi (3.14159...). Seem to have similarities with the C preprocessor #define but constants in Rust have a type so the compiler can check the usage for us.

## 2022-11-12

### Rust Update

Updated Rust to version 1.65 on home computer, using the shell command `rustup update`.
