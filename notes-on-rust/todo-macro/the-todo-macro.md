# TODO Macro

Rust have a dedicated notation for marking incomplete code by using a macro called `todo`.

```Rust
fn main() {
    // Display the message "Hello, world!"
    todo!("Display the message by using the println!() macro");
}
```

The code will fail in the way that is known as panic if a `todo` macro is encountered during execution.

```txt
thread 'main' panicked at 'not yet implemented: Display the message by using the println!() macro', src\main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\todo_macro.exe` (exit code: 101)
```
