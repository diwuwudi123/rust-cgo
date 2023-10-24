use std::fs;

#[no_mangle]
pub extern "C" fn hello_world() {
    let locale = fs::read("./test.txt").unwrap();

    println!("Hello, {}", String::from_utf8(locale).unwrap());
}
