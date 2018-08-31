extern crate libloading;
extern crate peripherio;

use peripherio::device::driver::Driver;

fn main() {
    let path = Driver::resolve("hello").unwrap();
    println!("{:?}", path);
    let driver = Driver::new(path).unwrap();
    println!("Good: {:?}", driver.validate());
}
