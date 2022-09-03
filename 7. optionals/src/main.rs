#![deny(clippy::all)]
#![allow(clippy::unnecessary_lazy_evaluations)]

fn main() {
  let name: Option<&str> = Some("John Doe");
  let mut age: Option<i32> = Some(12);
  let occupation: Option<&str> = None;

  // matching an option
  match name {
    Some(name) => println!("Hello, {}!", name),
    None => println!("No name"),
  }

  // matching option as mutable
  match age.as_mut() {
    Some(age) => *age += 1,
    None => {}
  }

  // unwrapping an option (with expect) if the value is None then the code execution panics
  let unwrapped_name = name.expect("name was not providered");
  println!("Name is {}", unwrapped_name);

  // unwrapping an option (with unwrap)
  let unwrapped_age = age.unwrap();
  println!("Age is {}", unwrapped_age);

  // unwrap multiple options at a time
  if let (Some(user_name), Some(user_age)) = (name, age) {
    println!("My name is {}. I am {} years", user_name, user_age);
  }

  // unwrapping with a default
  let unwrapped_occupation = occupation.unwrap_or("student");
  println!("Occupation is {}", unwrapped_occupation);

  let unwrapped_occupation = occupation.unwrap_or_else(|| {
    // do some stuff here
    // return default value
    "Programmer"
  });
  println!("Occupation is {}", unwrapped_occupation);

  let unwrapped_occupation = occupation.unwrap_or_default();
  println!("Occupation is {:#?}", unwrapped_occupation);

  // check if an optional has a value
  if name.is_some() {
    println!("There is a value");
  }

  // check if an optional has no value
  if occupation.is_none() {
    println!("There is no value");
  }

  // mapping over an option
  let mapped_age = age.map(|age| age * 2);
  println!("Age multiplied by 2 is: {}", mapped_age.unwrap_or_default());
}
