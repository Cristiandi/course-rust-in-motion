enum Clock {
  Sundial(u8),
  Digital(u8, u8),
  Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
  match clock {
    Clock::Sundial(hours) => 
      println!("It is about {} o'clock.", hours),
    Clock::Digital(hours, minutes) =>
      println!("It is {} minutes pass {}.", minutes, hours),
    Clock::Analog(hours, minutes, seconds) =>
      println!("It is {} minutes and {} seconds past {} o'clock.", hours, minutes, seconds)
  }
}

fn main () {
  tell_time(Clock::Sundial(10));
  tell_time(Clock::Digital(9, 4));
  tell_time(Clock::Analog(10, 14, 54));
}