use std::error::Error;
use anyhow::anyhow;
use rhai::Map;

#[macro_export]
macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

#[macro_export]
macro_rules! try_cast_map {
    ($value:expr, $err:expr) => {
        $value.clone().try_cast::<rhai::Map>()
            .ok_or(anyhow!($err))
    };
}

