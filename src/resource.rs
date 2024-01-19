#[macro_export]
#[cfg(all(target_os = "macos", app_bundle))]
macro_rules! resource {
    ($rel_path: expr, $path_in_resources: expr) => {{
        static FILE: ::std::sync::LazyLock<Vec<u8>> = ::std::sync::LazyLock::new(|| {
            ::std::fs::read($path_in_resources).expect(concat!(
                "Couldn't read resource at `",
                $path_in_resources,
                "`"
            ))
        });
        FILE.as_ref() as &[u8]
    }};
}

#[macro_export]
#[cfg(all(target_os = "macos", app_bundle))]
macro_rules! resource_str {
    ($rel_path: expr, $path_in_resources: expr) => {{
        static FILE: ::std::sync::LazyLock<String> = ::std::sync::LazyLock::new(|| {
            ::std::fs::read_to_string($path_in_resources).expect(concat!(
                "Couldn't read text resource at `",
                $path_in_resources,
                "`"
            ))
        });
        FILE.as_ref() as &str
    }};
}

#[macro_export]
#[cfg(not(all(target_os = "macos", app_bundle)))]
macro_rules! resource {
    ($rel_path: expr, $path_in_resources: expr) => {
        include_bytes!($rel_path)
    };
}

#[macro_export]
#[cfg(not(all(target_os = "macos", app_bundle)))]
macro_rules! resource_str {
    ($rel_path: expr, $path_in_resources: expr) => {
        include_str!($rel_path)
    };
}