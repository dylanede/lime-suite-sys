extern crate cmake;

fn main() {
    let dst = cmake::Config::new("LimeSuite")
        .define("BUILD_SHARED_LIBS", "0")
        .define("ENABLE_DESKTOP", "0")
        .define("ENABLE_EXAMPLES", "0")
        .define("ENABLE_LIME_UTIL", "0")
        .define("ENABLE_STREAM", "1")
        .define("ENABLE_SOAPY_LMS7", "0")
        .define("ENABLE_EVB7COM", "1")
        .define("ENABLE_GUI", "0")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=LimeSuite");
}