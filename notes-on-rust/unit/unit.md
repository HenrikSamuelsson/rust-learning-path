# Unit Type

Rust have a primitive type denoted by `()`, that goes by the name *unit*.

The unit can by design not carry any information, it has exactly one value `()`.

The purpose of the unit type is to have a value to use as result type when there is no other more meaningful value to use.

A function can return `()`. It also so that a function that leaves out the return type will implicitly return `()`.

Minimal example function called `foo` that will return `()`:

```Rust
fn foo()->() {}
```

## Reference

https://stackoverflow.com/a/75197630/686720
