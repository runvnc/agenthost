



#[macro_export]
macro_rules! s {
    ($s:expr) => { $s.to_string() }
}

#[macro_export]
macro_rules! dyn_map {
    ($value:expr, $err:expr) => {
        $value.clone().try_cast::<rhai::Map>()
            .ok_or(anyhow!($err))
    };
}

#[macro_export]
macro_rules! dyn_str {
    ($map:expr, $prop_name:expr) => {
        match $map.get($prop_name).map(|v| v.clone().as_string()) {
            Some(Ok(s)) => Ok(s),
            Some(Err(_)) | None => Err(anyhow::anyhow!("Failed to get '{}' as a string", $prop_name)),
        }
    };
}

