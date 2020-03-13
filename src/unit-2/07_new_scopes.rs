fn main() {
  let mut list = vec![1, 2, 3];

  // create an inner scope to use / create a mutable reference
  {
      let list_first = list.first();
      let list_last = list.last();

      println!(
          "The first element is {:?} and the last is {:?}",
          list_first,
          list_last,
      );
  }

  *list.first_mut().expect("list was empty") += 1;
}
