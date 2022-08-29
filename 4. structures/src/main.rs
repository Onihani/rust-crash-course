#![deny(clippy::all)]

struct Person {
  name: String,
  age: u8,
}

// Point 6 in action
// adding debeg trait to a struct
#[derive(Debug)]
struct Point(f64, f64, f64);

// Point 7 in action
impl Point {
  fn describe(&self) {
    println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
  }

  // static method
  fn zero() -> Point {
    Point(0.0, 0.0, 0.0)
  }
}

impl Point {
  fn double(&self) -> Point {
    Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
  }

  fn update_x_cordinate(&mut self, value: f64) {
    self.0 = value;
  }

  fn update_y_cordinate(&mut self, value: f64) {
    self.1 = value;
  }

  fn update_z_cordinate(&mut self, value: f64) {
    self.2 = value;
  }
}

fn main() {
  let person = create_person("Nat".to_string(), 30);
  let _other_person = Person {
    name: String::from("Gloria"),
    ..person
  };

  // getiing data from an instance of a struct
  println!("{} is {} years old", person.name, person.age);

  let mut point = Point::zero();
  point.update_x_cordinate(0.5);
  point.update_y_cordinate(0.2);
  point.update_z_cordinate(0.4);
  println!("{:?}", point);
  let point2 = point.double();
  point2.describe();
}

// Point 4 in action
fn create_person(name: String, age: u8) -> Person {
  Person { name, age }
}

// 📝 Note:
// 1. Structures are kind of classes in rust
// 2. We can create new instances of a structure
// 3. Dot notation is used to access data in an instance of a structure
// 4. If the names of variables to be used as values to initialize an instance of a new structure is the same as the fields of the structure we can parse the variable
// 5. Structure update syntax: more like spread and rest in js
// 6. We can create a Tuple Stuct in rust
// 7. Implentation blocks: structures can have logic and functions in them, usually this implementations are used to describe the structures (more like added methods to the structure)
// 8. Structures can have traits
// 9. Structures can have multiple implementations
