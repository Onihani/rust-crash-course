#![deny(clippy::all)]

// struct properties lifetime
struct _Person<'a> {
  first_name: &'a str,
  last_name: &'a str,
}

// implementation for structures with a lifetime
impl<'a> _Person<'a> {
  fn _first_char_in_first_name(&self) -> &str {
    &self.first_name[0..1]
  }
}

// lifetimes for enums
enum _Animal<'a> {
  Dog { name: &'a str },
}

fn main() {
  println!("Hello, world!");
}

// function return value lifetimes
fn _get_full_name() -> &'static str {
  "John Doe"
}

// function parameter life times
fn _get_random_name<'l1>(_a: &'l1 str, b: &'l1 str) -> &'l1 str {
  b
}

// default lifetimes
// here whatever life time the param name has will be the life time of the return value name
fn _get_first_name(name: &str) -> &str {
  name
}

// 📝 Notes:
/* Lifetime Rules */
// 1.
// 2. if you have a single param th
// 3.
