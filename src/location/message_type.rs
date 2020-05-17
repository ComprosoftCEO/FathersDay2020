pub use crate::item::Item;

pub enum MessageType {
  Generic(String),
  ItemCollected(Item),
  PersonTalking(String, String),
  NoLocation(String),
  NoItem(String),
  NotInInventory(String),
  CantUseItem(String),
  NoPerson(String),
  CantTalkTo(String),
  CantGiveItem(String, String),
}

impl ToString for MessageType {
  fn to_string(&self) -> String {
    match self {
      MessageType::Generic(s) => s.into(),
      MessageType::ItemCollected(i) => format!("Collected item '{}'", i.to_string()),
      MessageType::PersonTalking(person, msg) => format!("{}: {}", person, msg),
      MessageType::NoLocation(l) => format!("No location '{}'", l),
      MessageType::NoItem(i) => format!("No such item '{}' in this location", i),
      MessageType::NotInInventory(i) => format!("Item '{}' not in inventory", i),
      MessageType::CantUseItem(i) => format!("Can't use item '{}' in this location", i),
      MessageType::NoPerson(p) => format!("No such person '{}'", p),
      MessageType::CantTalkTo(p) => format!("Can't talk to {}", p),
      MessageType::CantGiveItem(p, i) => format!("Can't give item '{}' to {}", i, p),
    }
  }
}
