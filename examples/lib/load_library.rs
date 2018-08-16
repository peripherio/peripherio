extern crate rami;
extern crate libloading;

use rami::device::loader::Loader;

fn main() {
    let loader = Loader::new("lib").unwrap();
    println!("{:?}", loader.path());
    let func = unsafe { loader.get::<fn(u32) -> u32>("init").unwrap() };
    println!("{:?}", func(10));
}

