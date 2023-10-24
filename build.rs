extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::from_file("./cbindgen.toml");
    println!("res is {:?}", config);
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config.unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("./build/include/test.h");
}
