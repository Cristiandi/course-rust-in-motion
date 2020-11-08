pub struct Stats {
  hp: u8,
  sp: u8,
}

pub struct Monster {
  stats: Stats,
  friends: Vec<Friend>,
}

pub struct Friend {
  loyalty: u8,
}

impl Monster {
  pub fn final_breath(&mut self) {
      if let Some(friend) = self.friends.first() {
          self.stats.heal(friend.loyalty);
          println!("Healing for {}", friend.loyalty);
      }
  }
}

impl Stats {
  pub fn heal(&mut self, amount: u8) {
      self.hp += amount;
      self.sp -= amount;
  }
}

fn main() {
  let stats1: Stats = Stats { hp: 100, sp: 100 }; 

  let friend1: Friend = Friend { loyalty: 10 };
  let friend2: Friend = Friend { loyalty: 9 };
  let friend3: Friend = Friend { loyalty: 8 };

  let friend_list = vec![friend1, friend2, friend3];

  let mut monster1: Monster = Monster { stats: stats1, friends: friend_list };

  monster1.final_breath();

  println!("current monster hp {}", monster1.stats.hp);
  println!("current monster sp {}", monster1.stats.sp);
}
