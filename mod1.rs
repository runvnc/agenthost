// mod1.rs

// Define a macro
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}