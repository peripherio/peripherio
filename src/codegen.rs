extern crate rami;

use rami::driver::driver::DriverData;

fn ctype(type_str: &str) -> String {
    match type_str {
        "number" => "double",
        "integer" => "long",
        "string" => "const char*",
        _ => unimplemented!()
    }.to_string()
}

fn main() {
    let drv = DriverData::new(".").unwrap();
    println!("typedef struct Config_ {{");
    for (key, val) in drv.schemas() {
        println!("  {} {};", ctype(val["type"].as_str().unwrap()), key.replace(".", "_"));
    }
    println!("}} __attribute__((__packed__)) Config;");
}

