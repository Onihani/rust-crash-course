// set up clippy for linting
#![deny(clippy::all)]

// constants
// should have a data type and a value when initialized
//  cannot be reassigned
const LESSON_NUMBER: u8  = 1;

fn main() {
  println!("LESSON NUMBER: {}\n", LESSON_NUMBER);

  // mut allows a variable to be mutable
  let mut first_name = "Nat";
  // string
  let last_name = "Doe";
  // instegers: can be hex values or have underscores 9_000_000
  let age = 30;
  // boolean
  let reassign_name = true;
  // float
  let height = 6.2;
  // tupples (collection of related data)
  let personal_data = (age, first_name);

  // ❌ error: variables by default are not mutable
  // first_name = "John";

  // ✅ success: rust allows reassigning here becaase first_name is mutable
  if reassign_name {
    first_name = "Gloria";

    // ❌ error: variables cannot be reassigned to a different data type
    // first_name = 69;
  }

  // retriving data from tuples
  let (user_age, _user_name) = personal_data;
  println!("Your name is {} {}", personal_data.1, last_name);

  // variable shadowing: 
  // allows reassigning to a variable (new data could have a different data type)
  let last_name = 69;

  println!(
    "{} {} are {} years old and {} meters in height",
    first_name, last_name, user_age, height
  );
}
