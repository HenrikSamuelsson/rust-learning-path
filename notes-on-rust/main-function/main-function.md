# The Main Function

A function is a block of code that does a specific task. The code becomes easier to maintain and understand by separating the code into different functions that perform dedicated tasks. A function can call other functions causing a chain of tasks to be executed.

There must be a convention on what to call the very first function that shall run when a program is started. This special function is the starting point of the chain of functions that runs during program execution. In Rust, and in many other programming languages, the convention is to call this function `main`.

The Rust compiler `rustc` is aware of the convention to use main as the starting point and will organize the code in the executable program for us so that `main` function becomes the starting point.

Not every piece of Rust code is intended to be used as an executable program. Code can be developed with the just the intention to have it as library that other code can use. These library project does not need to have a `main` function.
