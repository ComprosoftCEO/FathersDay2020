#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
  Ladder,
  Whistle,
  Amulet,
  Log,
  OldKey,
  Armor,
}

impl ToString for Item {
  fn to_string(&self) -> String {
    match self {
      Item::Ladder => "ladder".into(),
      Item::Whistle => "whistle".into(),
      Item::Amulet => "amulet".into(),
      Item::Log => "log".into(),
      Item::OldKey => "oldkey".into(),
      Item::Armor => "armor".into(),
    }
  }
}
