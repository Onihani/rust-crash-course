#![deny(clippy::all)]

use std::f64::consts::PI;

#[derive(PartialEq)]
enum AnimalType {
  Cat,
  Dog { name: String },
  _Rabbit,
}

enum Shapes {
  _Circle { radius: f64, _center: (f64, f64) },
  Rectangle { length: f64, breadth: f64 },
  Square(f64, f64, Size),
}
struct Size {
  length: f64,
}

impl Shapes {
  fn area(&self) -> f64 {
    match self {
      Shapes::_Circle { radius, _center } => PI * radius * radius,
      Shapes::Rectangle { length, breadth } => length * breadth,
      Shapes::Square(_x, _y, Size { length }) => length * length,
    }
  }
}

fn main() {
  let fluffy = AnimalType::Cat;
  match fluffy {
    AnimalType::Cat => println!("Meow!"),
    _ => println!("Something"),
    // AnimalType::Cat => println!("Meow!"),
    // AnimalType::Rabbit => println!("Hoot!"),
  }

  let rectangle = Shapes::Rectangle {
    length: 4.0,
    breadth: 5.0,
  };

  // if let Shapes::Rectangle { length, breadth } = rectangle {
  //   println!("length = {}, breadth = {}", length, breadth)
  // }

  match rectangle {
    Shapes::Rectangle { length, breadth } => println!("length = {}, breadth = {}", length, breadth),
    _ => println!("Not a rectangle"),
  }

  let square = Shapes::Square(2.0, 4.0, Size { length: 5.0 });
  // if let Shapes::Square(x, y, Size{length}) = square {
  //   println!("{} {} {}", x, y, length)
  // }

  match square {
    Shapes::Square(x, y, Size { length }) => println!("{} {} {}", x, y, length),
    _ => println!("Not a square"),
  }

  println!("Area of square is: {}", square.area());

  let crane = AnimalType::Dog {
    name: String::from("Crane"),
  };
  let name = match crane {
    AnimalType::Dog { name } => name,
    _ => "Unnamed".to_string(),
  };
  println!("My pets name is {}", name);
}

// 📝 NOTES:
// Enums are a way to group similar items together
// Variable names for enums use pascal casing
