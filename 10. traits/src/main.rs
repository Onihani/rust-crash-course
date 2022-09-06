#![deny(clippy::all)]

use ::std::fmt;

#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} is {} years old",
      self.first_name, self.last_name, self.age
    )
  }
}

trait HasName {
  fn first_name(&self) -> &str;
  fn last_name(&self) -> &str;
}

trait HasFullName
where
  Self: HasName,
{
  fn full_name(&self) -> String;
}

impl<T> HasFullName for T
where
  T: HasName,
{
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name(), self.last_name())
  }
}

impl HasName for Person {
  fn first_name(&self) -> &str {
    &self.first_name
  }

  fn last_name(&self) -> &str {
    &self.last_name
  }
}

// trait HasFullName {
//   fn full_name(&self) -> String;
// }

// impl HasFullName for Person {
//   fn full_name(&self) -> String {
//     format!("{} {}", self.first_name, self.last_name)
//   }
// }

trait CanInitializeWithFullName {
  fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
  fn new(full_name: &str) -> Self {
    let parts: Vec<&str> = full_name.split(' ').collect();

    Person {
      first_name: parts[0].to_string(),
      last_name: parts[1].to_string(),
      age: 0,
    }
  }
}

trait CanRun {
  fn run(&self);
}

impl CanRun for Person {
  fn run(&self) {
    // todo
  }
}

fn main() {
  let person = Person::new("John Doe");

  println!("{}", person);

  print_full_name_and_age(&person);
  print_detials(&person);
}

fn print_full_name_and_age(value: &impl HasFullName) {
  println!("{}", value.full_name());
}

fn print_detials<T>(value: &T)
where
  T: HasFullName + CanRun,
{
  println!("{}", value.full_name());
  value.run();
}
