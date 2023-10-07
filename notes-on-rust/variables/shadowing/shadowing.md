# Variable Shadowing

Shadowing in Rust allows re-declaration of a variable in the same scope, using the same name.

Shadowing will create a new variable, at a new memory location. The original variable is no longer accessible, in the current scope, due to that the name will from now on be bound to the new shadowing variable.

Shadowing can be useful since there is in certain situations no need to come up with a new name for what is essentially the same thing, as illustrated in the following example.

TODO add example on shadowing

Shadowing will however also add some complexity to the code making it harder to follow for developers not familiar with the concept.

## References

[https://www.thorsten-hans.com/shadowing-temporary-mutability-rust/](https://www.thorsten-hans.com/shadowing-temporary-mutability-rust/)  
[https://reintech.io/blog/understanding-implementing-rust-shadowing](https://reintech.io/blog/understanding-implementing-rust-shadowing)  
