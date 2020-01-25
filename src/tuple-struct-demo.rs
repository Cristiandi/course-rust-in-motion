// tuple struct
struct Triangle(u32, u32, u32);

fn is_equirateral(triangle: Triangle) -> bool {
  triangle.0 == triangle.1 && triangle.1 == triangle.2
}

fn main() {
    let triangle1 = Triangle(3, 4, 5);
    
    println!("triangle1 is equirateral {}", is_equirateral(triangle1));
}