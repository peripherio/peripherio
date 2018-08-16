extern crate rami;

use rami::device::loader::Loader;

fn main() {
    let loader = Loader::new("lib").unwrap();
    println!("{:?}", loader.path());
}

