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
