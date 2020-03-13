fn main() {
  let mut list = vec![1, 2, 3];
  
  // can create an mutable reference / borrows  because is before the inmmutable referece / borrows
  *list.first_mut().expect("list was empty") += 1;

  let list_first = list.first();
  let list_last = list.last();

  println!(
      "The first element is {:?} and the last is {:?}",
      list_first,
      list_last,
  );
}
