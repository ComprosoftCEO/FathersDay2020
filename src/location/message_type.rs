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
