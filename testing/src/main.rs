#[macro_use]
extern crate curly_derive;

use curly::Provider;
//#[derive(Provider)]
//struct SomeData(String, u16);

fn main() {
    println!("Hello, world!");
    //    println!("{}", SomeData("abc".to_string(), 0).provide(curly::formatters::CurlyFormatter::default(), "0").unwrap());
}
