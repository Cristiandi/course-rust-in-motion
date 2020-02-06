enum HockeyPosition {
  Center,
  Wing,
  Defense,
  Goalie,
}

// attributed struct
struct HockeyPlayer {
  name: String,
  number: u8,
  position: HockeyPosition,
  goals_ytd: u8
}

enum Clock {
  Sundial { hours: u8 },
  Digital { hours: u8, minutes: u8 },
  Analog { hours: u8, minutes: u8, seconds: u8 },
}

fn main() {
    let mut player = HockeyPlayer {
      name: String::from("Cristian Ippolito"),
      number: 23,
      position: HockeyPosition::Wing,
      goals_ytd: 7
    };

    player.goals_ytd += 1;

    println!(
      "{} - {} has scored {} goals this season",
      player.name,
      player.number,
      player.goals_ytd
    );

    let clock = Clock::Analog {
      hours: 9,
      minutes: 25,
      seconds: 9 
    };

}