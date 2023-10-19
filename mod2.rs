// mod2.rs

// Import the macro from mod1
#[macro_use]
mod mod1;

// Use the macro
fn main() {
    say_hello!();
}