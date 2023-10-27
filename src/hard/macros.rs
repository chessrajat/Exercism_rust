#[macro_export]
macro_rules! hashmap {
    () => {{
        let _map = ::std::collections::HashMap::new();
        _map
    }};
    ($($key:expr => $value:expr),*) => {{
        let mut _map = ::std::collections::HashMap::new();
        $(
            _map.insert($key, $value);
        )*
        _map
    }};
    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
}
