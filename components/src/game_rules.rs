use flagset::flags;

flags! {
  enum GameRuleFlags: u8 {
    ALL
  }
}

pub struct GameRules {
  flag: GameRuleFlags
}