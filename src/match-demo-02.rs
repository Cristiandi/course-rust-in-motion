fn main() {
  let is_confirmed = true;
  let is_active = false;

  match (is_confirmed, is_active) {
    (true, true) => println!("this account is in good standing."),
    (false, true) => println!("need to confirm your account!"),
    (false, false) => println!("this account will be desactivated."),
    _ => println!("no controlled")
  }
}