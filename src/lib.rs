use std::fs;

#[no_mangle]
pub extern "C" fn hello_world() {
    let test = fs::read("./test.txt").unwrap();

    println!("Hello, {}", String::from_utf8(test).unwrap());
}
