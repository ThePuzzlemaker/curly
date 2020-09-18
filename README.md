# Curly

Type-safe runtime text formatting for humans.

> ## NOTICE
> 
> This library is currently work-in-progress and not in a usable state.

## Motivation

I haven't really seen many formatting libraries for Rust that aren't really HTML-centric. I was working on a project where I needed user-defined formatting at runtime, so I started this project. It eventually evolved to basically being a runtime text formatting library with similar (or sometimes equivalent) syntax to Rust's `format!` family of macros.

## Usage (currently doesn't actually work)

```rust
// With `#[cfg(feature = "derive")]`, we get the `Provider` derive macro
#[macro_use]
extern crate curly;

#[derive(Debug, Provider)]
struct SomeStruct {
    message: String,

    #[curly_ignore]
    some_internal_value: i16,

    #[curly_rename = "meaning_of_life"]
    the_meaning_of_life_the_universe_and_everything: u8,

    // This value is automatically ignored. Evenutally, I'll add something like `#[curly_include]` that automatically unignores it.
    _something: u32,
}

fn main() {
    let some_struct = SomeStruct {
        message: String::from("Hello, world!"),
        some_internal_value: 5, // This value can't be printed with the formatter
        the_meaning_of_life_the_universe_and_everything: 42,
        _something: 0, // Nor can this
    };

    let goodbye = "Goodbye! Thanks for reading this!";

    // Just imagine this is obtained from user input.
    let format_string = "{message} The meaning of life, the universe, and everything is {meaning_of_life}. {goodbye}";

    assert_eq!(
        curly!(
            format_string,
            goodbye: String = goodbye,
            ..some_struct: SomeStruct
        ),
        "Hello, world! The meaning of life, the universe, and everything is 42. Goodbye! Thanks for reading this!"
    );
}
```