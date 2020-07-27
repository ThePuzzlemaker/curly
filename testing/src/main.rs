#![warn(clippy::pedantic)]
// #[macro_use]
extern crate curly;

#[macro_use]
extern crate curly_derive;

#[derive(Debug, Provider)]
struct SomeStruct {
    value1: String,

    #[curly_ignore]
    internal_value_2: String,

    #[curly_rename = "something_shorter"]
    rename_this_really_long_name_to_be_something_shorter: String,

    _automatically_ignored: String,
}

fn main() {
    println!("Hello, world!");
}
