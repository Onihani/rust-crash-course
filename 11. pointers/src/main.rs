#![deny(clippy::all)]

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

// custom box implementation
struct BoxedValue<T> {
  value: T,
}

impl<T> BoxedValue<T> {
  fn new(value: T) -> Self {
    BoxedValue { value }
  }
}

impl<T> Deref for BoxedValue<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

// person struct
struct Person {
  name: String,
  age: Cell<u8>,
}

impl Person {
  fn increment_age(&self) -> u8 {
    self.age.set(self.age.get() + 1);
    self.age.get()
  }
}

fn main() {
  boxes();

  println!("\n\n\n");

  reference_counting();

  println!("\n\n\n");

  cell_pointers();

  println!("\n\n\n");

  reference_cell();
}

fn boxes() {
  let age = BoxedValue::new(21);
  let actual_age = *age;
  let twice = actual_age * 2;
  let ref_to_age = age.deref();
  let other = *(age.deref());

  println!("twice: {}", twice);
  println!("reference to age: {}", ref_to_age);
  println!("other: {}", other);

  print_integer(&age);
}

fn reference_counting() {
  let array = vec!["John".to_string(), "Jane".to_string()];
  let rc = Rc::new(array);
  let weak = Rc::downgrade(&rc);
  let rc2 = rc.clone();

  // dropping rc
  drop(rc);

  match weak.upgrade() {
    Some(rc) => println!("rc: {:?}", rc),
    None => println!("None"),
  };

  println!("rc2: {:?}", rc2)
}

fn cell_pointers() {
  let person = Person {
    name: "John".to_string(),
    age: Cell::new(20),
  };

  let person_new_age = person.increment_age();
  let person_age = person.age.get();

  println!("new age: {}", person_new_age);
  println!("person age: {}", person_age);
}

fn reference_cell() {
  let ref_cell = RefCell::new(vec![1, 2, 3]);

  // let mut mut_ref = ref_cell.borrow_mut();
  let len = ref_cell.borrow().len();

  println!("len: {}", len);
}

fn print_integer(&value: &i32) {
  print!("{}", value);
}
