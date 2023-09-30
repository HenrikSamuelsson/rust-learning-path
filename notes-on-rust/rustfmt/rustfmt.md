# rustfmt

Rust have a code formatter called *rustfmt*. The purpose of this tool is to have a standardized way to format Rust source code. The code becomes easier to read and understand when it always follows familiar formatting patterns.

A Rust installer, like rustup, will typically include rustfmt by default.

rustfmt can be run in a terminal by the use of cargo. Use the following command to format the code in the current crate:

```txt
cargo fmt
```

## Try rustfmt

As an example of running rustfmt start out with the following incorrectly formatted Rust source code. This code compiles but have multiple issues, incorrect indentation, trailing spaces, and strange placing of parentheses. But the code will still compile.

```rust
fn

main  (
    
)
        {   
println!         ("Hello, world!"  )   ;   
}
```

To fix the formatting of the above code, in a terminal window, navigate to the folder that holds the Cargo.toml for the code. When in this folder execute the command `cargo fmt`. This will cause cargo to run rustfmt.

The source code will formatted to the below version that conforms with the community guidelines on how to write Rust code.

```rust
fn main() {
    println!("Hello, world!");
}
```

## References

Klabnik S, Nichols C, (2023). *The Rust Programming language, 2nd Edition*, San Francisco: No Starch Press, Inc.
