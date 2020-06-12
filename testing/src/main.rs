#[macro_use]
extern crate curly_derive;

#[macro_use]
extern crate newcurly;
#[derive(Provider)]
struct SomeData {
    value2: bool,
}

fn main() {
    let fmt = "{{!q:value/!}}";
    let data = SomeData { value2: false };
    let value3: bool = false;
    curly!(
        fmt,
        value: bool = false,
        value3: bool = value3,
        ..data: SomeData
    );

    // let formatter = curly::formatters::CurlyFormatter::from_segment("{{!q:value/!}}").unwrap();
    // let data = SomeData {
    //     value: false,
    // };
    // let provided = data.provide(&formatter, "value").unwrap();
    // println!("{}", provided);
    // //    println!("{}", SomeData("abc".to_string(), 0).provide(curly::formatters::CurlyFormatter::default(), "0").unwrap());
}
