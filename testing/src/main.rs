#[macro_use]
extern crate curly_derive;

use curly::Provider;
#[derive(Provider)]
struct SomeData {
    value: bool,
}

fn main() {
    let formatter = curly::formatters::CurlyFormatter::from_segment("{{!q:value/!}}").unwrap();
    let data = SomeData {
        value: false,
    };
    let provided = data.provide(&formatter, "value").unwrap();
    println!("{}", provided);
    //    println!("{}", SomeData("abc".to_string(), 0).provide(curly::formatters::CurlyFormatter::default(), "0").unwrap());
}
