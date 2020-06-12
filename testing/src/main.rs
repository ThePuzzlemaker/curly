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
    let fmter = newcurly::formatters::CurlyFormatter::from_segment(fmt, 0, 0)
        .expect("failed to get formatter from segment");
    // let formatter = curly::formatters::CurlyFormatter::from_segment("{{!q:value/!}}").unwrap();
    // let data = SomeData {
    //     value: false,
    // };
    // let provided = data.provide(&formatter, "value").unwrap();
    // println!("{}", provided);
    // //    println!("{}", SomeData("abc".to_string(), 0).provide(curly::formatters::CurlyFormatter::default(), "0").unwrap());
}
