#![deny(clippy::all)]

fn main() {
  let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");
  let error: Result<&str, ()> = Err(());

  match value {
    Ok(value) => println!("value: {}", value),
    Err(error) => println!("error: {}", error),
  }

  match error {
    Ok(value) => println!("value: {}", value),
    Err(_) => println!("error occurred"),
  }

  let is_ok = get_user_name(false).is_ok();
  let user_name = get_user_name(false).expect("Failed to get username");
  println!("username: {}, is ok: {}", user_name, is_ok);

  let is_err = get_user_name(true).is_err();
  let user_name_err = get_user_name(true).expect_err("I didn't expect a user name");
  println!("username err: {}, is err: {}", user_name_err, is_err);

  let full_name = get_full_name();
  match &full_name {
    Ok(name) => println!("Hello, {}", name),
    Err(error) => println!("Oops an error occured: {}", error),
  }
  let name_err_length = full_name.map_err(|s| s.len()).unwrap_err();
  println!("full name err length is: {}", name_err_length);
}

fn get_user_name (has_error: bool) -> Result<String, String> {
  if has_error {
    return Err("invalid user name".to_string());
  }

  Ok("Nat".to_string())
}

fn get_first_name () -> Result<String, String> {
  Err("invalid first name".to_string())
}

fn get_last_name () -> Result<String, String> {
  Ok("Doe".to_string())
}

fn get_full_name () -> Result<String, String> {
  let first_name = get_first_name()?;
  let last_name = get_last_name()?;

  Ok(format!("{} {}", first_name, last_name))
}