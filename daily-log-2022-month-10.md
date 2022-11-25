# Daily Log October 2022

Notes on the daily steps taken during completion of this Rust Learning Path project.

## 2022-10-22

### Learning Rust for Embedded Systems

Read the blog [Learning Rust for Embedded Systems](https://www.embeddedrelated.com/showarticle/1432.php) and downloaded the pdf version for future reference. Stored pdf version on my personal google drive in a folder called learning-rust.

Two takeaways from this blog. Firstly the list of new concepts in Rust and their mapping to concepts in other programming languages. Secondary is the linked resources in the blog that will be used for future exploration of Rust.

## 2022-10-23

### Why Rust won't replace C

Watched the video [Why Rust won't replace C (just yet anyway)](https://youtu.be/ojEXMM_1bVA) on YouTube.

Well balanced discussion on benefits and challenges if to move away from C for embedded projects and instead use Rust.

Benefits are that Rust is safer, built in support of elements that requires third party tools such as static code checkers, good support for multi threaded applications. Challenges is that it requires the developers to learn a new language and possibly also start using new tools, can be hard to get the team and management motivated and understand the long term benefits.

## 2022-10-24

### Installation Work Computer

Rust can be installed on a Windows machine by following the instructions at [Rust - Install Rust](https://www.rust-lang.org/tools/install).

Turned out that I had installed Rust already on my work computer.

```txt
$ rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
```

## 2022-10-25

### The Rust Programming Language

Decided that the first book to read will be [The Rust Programming Language](https://doc.rust-lang.org/book/). This is a free resource published online in html format. Also found a pdf version online that I found more readable due to being a "latex-style" formatted.

## 2022-10-29

## Guessing Game

Implemented the classical number guessing game in Rust as described in chapter 2 of The Rust Programming Language book.

Learned about:

- Console input and output
- Number parsing from text
- Comparison of numbers
- Setting library dependencies in the Cargo.toml file
- Using third party library crates
- Generation of random numbers in Rust
- Looping
- Match expressions

Cannot personally remember seeing something similar to the match expression in other programming languages so this was interesting.

> A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn.

## 2022-10-30

### The Embedded Rust Book

Found out about the online book [The Embedded Rust Book](https://docs.rust-embedded.org/book/). This is an An introductory book about using the Rust on "bare metal" embedded systems, such as microcontrollers. Plan is to read this book.

## 2022-11-25

### Slow Progress

Need some motivation why to get going with this again, here is a list:

- Can use Rust for competitive coding
- Do not need to use C in projects that slow me down due to being error prone
- Great for the brain to learn new things in general
