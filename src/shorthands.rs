use std::error::Error;

#[macro_export]
macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

pub type Res<T> = Result<T, Box<dyn Error>>;


