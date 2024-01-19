fn main() {
    if option_env!("BUILD_APP_BUNDLE").is_some() {
        println!("cargo:rustc-cfg=app_bundle");
    }
}
