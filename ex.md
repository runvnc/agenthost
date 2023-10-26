# Fixing Rust Error: No Method Named `get` Found for Struct `Arc<Mutex<HashMap>>`

The error message is indicating that the `get` method is not available for the `Arc<Mutex<HashMap>>` struct. This is because `Arc<Mutex<T>>` does not directly expose the methods of the inner type `T`. Instead, you need to lock the `Mutex` first to get a mutable reference to the inner `HashMap`, and then you can call `get` on it.

Here's how you can do it:

```rust
let users_lock = users.lock().unwrap();
let tx = users_lock.get(&my_id);
```

In this code, `users.lock().unwrap()` locks the `Mutex` and returns a `MutexGuard` that dereferences into `HashMap`. The `unwrap` is used to handle the potential error that `lock` might return (it returns a `Result`). Be aware that if another thread panicked while holding this lock, the call to `lock` will panic.

As for your second question, in Rust, you don't need to manually unlock or release the lock when done. The `MutexGuard` returned by `lock` automatically releases the lock when it goes out of scope.