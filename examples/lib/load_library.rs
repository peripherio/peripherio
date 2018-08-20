extern crate rami;
extern crate libloading;

use rami::device::loader::Loader;

fn main() {
    let path = Loader::resolve("hello").unwrap();
    println!("{:?}", path());
    let loader = Loader::new(path).unwrap();
    println!("Good: {:?}", loader.validate());
    let func = unsafe { loader.get::<fn(u32) -> u32>("init").unwrap() };
    println!("{:?}", func(10));
}

