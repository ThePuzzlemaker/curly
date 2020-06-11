#[derive(Provider)]
struct SomeValuePair{
  value1: String,
  value2: u64
}

fn main() {
  let user_input = "{{value1}}, {{value2}} ({{string}})";
  let output = curly!(user_input, ..SomeValuePair, string="Thanks!");
}