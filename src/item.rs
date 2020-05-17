use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Item {
  Ladder,
  Whistle,
  Amulet,
  Log,
  OldKey,
  Armor,
  Pitchfork,
  Shield,
  Sword,
}

impl Item {
  pub fn find_string(s: &str) -> Option<Item> {
    Item::iter().find(|i| i.to_string() == s)
  }
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
      Item::Pitchfork => "pitchfork".into(),
      Item::Shield => "shield".into(),
      Item::Sword => "sword".into(),
    }
  }
}
