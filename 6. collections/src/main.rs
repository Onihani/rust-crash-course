#![deny(clippy::all)]

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Person {
  name: String,
  age: u8,
}

fn main() {
  println!("=========Tuples=========");
  tuples();
  println!("\n");

  println!("=========Vectors=========");
  vectors();
  println!("\n");

  println!("=========Hashmaps=========");
  hashmaps();
  println!("\n");

  println!("=========Iterators=========");
  iterators();
  println!("\n");
}

// a function that returns a tuple
fn get_values() -> (String, String, i32) {
  ("Hello".to_string(), "World".to_string(), 30)
}

// tuples
fn tuples() {
  // creating a tuple
  let values = get_values();

  // unpacking tuples (dot notation)
  let value1 = values.0;
  let value2 = values.1.to_string();
  let _age = values.2;

  // unpacking tuples (destructuring)
  // use underscores to ignore a value in a tuple
  let (_, value_2, _) = values;

  println!("{}, {}!", value1, value2);
  println!("{}, {}!", value1, value_2);
}

// vectors
fn vectors() {
  // creating a vector (fixed sizes vector)
  let values: [&str; 2] = ["foo", "bar"];

  // we can iterate over the values in a vector
  for (index, value) in values.iter().enumerate() {
    println!("iteration {}: {}", index, value);
  }

  // getting data from a vector
  let value1 = &values[0];
  println!("value1: {}", value1);

  // getting the length of a vector
  let length = values.len();
  println!("vector length: {}", length);

  // mapping a vector
  let mapped_values:Vec<String> = values.iter().map(|val| format!("{} mapped", val)).collect();
  println!("mapped_values: {:?}", mapped_values);

  // creating a dynamic sized vector (with the vector macro)
  let mut values1 = vec![1, 2, 3];
  let mut values2 = vec![4, 5, 6];

  // pushing values to a dynamic vector
  values1.push(4); // -> [1, 2, 3, 4]

  // popping values from a dynamic vector
  values1.pop(); // -> [1, 2, 3]

  // remove all values from a vector
  values1.clear(); // -> []

  // concat/copy values from one vector to another
  values1.extend_from_slice(&values2); // -> [4, 5, 6]

  // move values from one vector to another
  values1.append(&mut values2);
  // values1 becomes -> [4, 5, 6, 4, 5, 6]
  // values2 becomes -> []

  // check if a vector contains a particular value
  if values1.contains(&4) {
    println!("Yes")
  } else {
    println!("No")
  }

  // check if a vector is empty
  if values2.is_empty() {
    println!("empty")
  } else {
    println!("not empty")
  }

  println!("values1: {:?}", values1);
  println!("values2: {:?}", values2);
}

fn hashmaps() {
  // creating a hashmap
  let mut values: HashMap<&str, &str> = HashMap::new();

  // inserting values into a hashmap
  values.insert("name", "Nat");
  values.insert("gender", "male");
  values.insert("foo", "bar");

  // check if the hashmap contains a key
  if values.contains_key("name") {
    println!("key name exists");
  } else {
    println!("key name doesn't exist");
  }

  // remove key and value
  values.remove("foo");

  // unsafely read values from a hash map (panics if key does not exist)
  let name = values["gender"];
  println!("name: {}", name);

  // safely read values from a hash map
  match values.get("name") {
    Some(value) => println!("name: {:#?}", value),
    None => println!("Not found"),
  };

  // iterate over the key and value of a hashmap
  for (&key, &value) in &values {
    println!("{}: {}", key, value);
  }

  // get an entry from a hashmap
  let entry = values.entry("gender");
  println!("entry: {:#?}", entry);
  // use case of entry
  values.entry("city").or_insert("Accra");

  println!("{:?}", values);
}

fn iterators() {
  let values = vec![1, 2, 3, 4, 5];
  let iter = values.iter();

  // summing iterators
  let sum: i32 = iter.sum();
  println!("sum: {}", sum);

  // mapping iterators
  let mapped: Vec<i32> = values.iter().map(|v| v * 2).collect();
  println!("mapped: {:#?}", mapped);

  let names = vec!["John", "Jane", "Mary", "Bob", "Tom"];
  // filter iterations
  for name in names.iter().filter(|name| name.len() >= 4) {
    println!("filtered name: {}", name);
  }

  // owned iter values (into_iter)
  // jump over iterations with continue
  // break iterations with break
  for name in names.into_iter() {
    if name.len() >= 4 {
      continue;
    }
    if name.len() == 3 {
      break;
    }
    println!("name: {}", name);
  }
}

// Note:
// 1. Tuples are a collection of data (unrelated data)
// 2. Vectors are the equivalent of arrays in rust
// 2. Vectors are usually two dimensional arrays of data
// 3. Hash maps are key - value pairs
// 3. Hash maps are the equivalent of dictionaries/objects in other programming languages
// 4. Iterators
