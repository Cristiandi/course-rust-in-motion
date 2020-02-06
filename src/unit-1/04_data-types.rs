fn main() {
  let flag : bool = false;
  println!("I'm a bool {}", flag);

  let n1 : i8 = -5;
  println!("I'm a i8 {}", n1);
  let n2 : i16 = -50;
  println!("I'm a i16 {}", n2);
  let n3 : i32 = -1500;
  println!("I'm a i32 {}", n3);
  let n4 : i64 = -2500;
  println!("I'm a i32 {}", n4);
  let n5 : i128 = -2000;
  println!("I'm a i32 {}", n5);
  let n6 : isize = -4000;
  println!("I'm a isize {}", n6);

  let n7 : u8 = 5;
  println!("I'm a u8 {}", n7);
  let n8 : u16 = 50;
  println!("I'm a u16 {}", n8);
  let n9 : u32 = 1500;
  println!("I'm a u32 {}", n9);
  let n10 : u64 = 2500;
  println!("I'm a u64 {}", n10);
  let n11 : u128 = 2000;
  println!("I'm a u128 {}", n11);
  let n12 : usize = 4000;
  println!("I'm a usize {}", n12);

  let n13 : f32 = 1.323;
  println!("I'm a f32 {}", n13);
  let n14 : f64 = 1.32332432423;
  println!("I'm a f64 {}", n14);

  let c : char = '«';
  println!("I'm a char {}", c);

  let tup = (1, 'c', true);
  // let (x, y, z) = tup;
  let tup_first = tup.0;
  let tup_second = tup.1;
  println!("the tup_first is {}", tup_first);
  println!("the tup_second is {}", tup_second);

  let array = [0.0, 3.14, -8.7928];
  let array_second = array[1];
  println!("the array_second is {}", array_second);

  // slice
  let a = [100, 200, 300];
  let _b = &a[0..1];
}