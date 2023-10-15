//! Help for Compilation Error E0382

This error occurs when you try to borrow a value after it has been moved. In Rust, values are moved when they are used as function arguments or assigned to another variable. After a value has been moved, it can no longer be used in the original variable.

In your case, the `engine` value is moved when it is used to create the `handler` instance (line 118). Therefore, you cannot use `engine` again on line 138 because it has already been moved.

To fix this error, you can borrow the `engine` from the `handler` instance instead of trying to use the moved `engine`. Replace `engine.parse_json(&arg_, true).unwrap_or(Map::new());` with `handler.engine.parse_json(&arg_, true).unwrap_or(Map::new());`.

Here's the corrected code:

```rust
let argmap = handler.engine.parse_json(&arg_, true).unwrap_or(Map::new());
```

This will borrow the `engine` from the `handler` instead of trying to use the moved `engine`.