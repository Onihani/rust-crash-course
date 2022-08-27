// set up clippy for linting
#![deny(clippy::all)]

fn main() {
  let first_name = String::from("Nat");
  // moving:
  // the value of first_name is moved into last_name here
  let last_name = first_name;

  // ❌ error: first_name is not accessible here as it has been moved into last_name
  // println!("Your first name is {}", first_name);

  println!("Your last name is {}", last_name); // last_name = Nat;

  // borrowing:
  // ✅ success: here name now has (a read only) reference pointing to first_name in memory
  // making both accessible
  let name = &last_name;

  println!("Your last name is {}", last_name); // last_name = Nat;
  println!("Your name is {}", name); // last_name = Nat;

  greet(&last_name);
  // here no need to parse a reference of name because it's already a reference
  greet(name);

  // ❌ error: because name here is a read only reference
  // empty_string(name);
  // ❌ error: because we parsed a mutable reference of last_name but last_name is not mutable
  // empty_string(&mut last_name);
  // ✅ success:
  // the value of last_name is moved into mutable_name
  // mutable_name is mutable
  let mut mutable_name = last_name;
  empty_string(&mut mutable_name);

  // ❌ error: last_name is not accessible here as it's value has been moved into mutable_name
  // println!("Your name is {}", last_name);

  let mutable_name1 = &mut mutable_name;
  mutable_name1.push_str("Hello");

  let mutable_name2 = &mut mutable_name;
  mutable_name2.push_str(" World!");

  // ❌ error: cannot have more than one mutable reference to a variable
  // mutable_name1 is inaccessible here
  // println!("mutable_name 1 {}", mutable_name1);

  // ✅ success: mutable_name2 is the only accessible mutable reference of mutable_name
  println!("mutable_name 2 {}", mutable_name2);

  let immutable_name = &mutable_name;
  // ❌ error: cannot have a mutable and an immutable reference to a variable at the same time
  // mutable_name2 becauses inaccessible here
  // println!("mutable_name 2 {}", mutable_name2);

  // ✅ success: mutable_name2 is the only accessible (read only) reference of mutable_name
  // we can have more read only references at a time tho
  println!("immutable_name {}", immutable_name);
}

// paramter name: borrows a read only reference
fn greet(name: &String) {
  println!("Hello, {}!", name);
}

// parameter value: borrows a mutable referance of value
fn empty_string(value: &mut String) {
  value.clear();
}

// creates a name (string) and returns it
fn _get_name() -> String {
  String::from("Gloria")
}
