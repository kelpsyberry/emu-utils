pub fn setup_current_dir() {
    #[cfg(all(target_os = "macos", app_bundle))]
    {
        use cocoa::{
            base::{id, nil},
            foundation::{NSBundle, NSString},
        };
        use std::{env::set_current_dir, ffi::CStr};
        let path = (|| unsafe {
            let main_bundle = id::mainBundle();
            if main_bundle == nil {
                return None;
            }
            let resource_path = main_bundle.resourcePath();
            if resource_path == nil {
                return None;
            }
            let result = CStr::from_ptr(resource_path.UTF8String())
                .to_str()
                .ok()
                .map(str::to_string);
            let _: () = msg_send![resource_path, release];
            result
        })()
        .expect("Couldn't get bundle resource path");
        set_current_dir(path).expect("Couldn't change working directory to bundle resource path");
    }
}

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