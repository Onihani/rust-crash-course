// set up clippy for linting
#![deny(clippy::all)]

fn main() {
  // inline function
  let say_hello_to = |name: &str| format!("Hello, {}!", name);
  let hello_message = say_hello_to("Gloria");

  let message = greet_person("Nat");
  print_message(&message);
  print_message(&hello_message);

  // Inline function with curly brackets
  let ask_for_age = || {
    let mut age = String::new();
    println!("How old are you ?");
    std::io::stdin().read_line(&mut age).expect("Failed to read input");
    let age = age.trim().parse::<u32>().expect("Please type a number");

    age + 10
  };

  let user_age = ask_for_age();
  println!("You are going to be {} in 10 years time", user_age);

  // here _ptr is a pointer to ask_for_age
  let _ptr = ask_for_age;
  // ask for age is still accessible here
  ask_for_age();
}

// function with no return typw
fn print_message(message: &str) {
  println!("{}", message);
}

// function that returns a sring
fn greet_person(to_person: &str) -> String {
  format!("Hello, {}", to_person)
}

// function that accepts another function as parameter
fn _process_name(name: &str, callback: fn(&str)) {
  // do something here
  callback(name);
}

// 📝 Note:
// - Functions can be declared in other functions
// - Inline Functions can have curly brackets
// - Create a pointer to a Function by assigning the function as value to a variable
// - A Function can be parsed as a paramenter to another function
