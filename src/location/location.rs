use crate::action::Action;
use crate::item::Item;
use crate::state::State;

pub trait Location {
  fn get_image(&self, state: &State) -> String;
  fn move_to(&self, state: &mut State, direction: &str) -> Option<Result<Box<dyn Location>, String>>;
  fn pickup_item(&self, state: &mut State, item: &str) -> Option<String>;
  fn use_item(&self, state: &mut State, item: &str) -> Option<String>;
  fn talk_to(&self, state: &mut State, person: &str) -> Option<String>;
  fn give_to(&self, state: &mut State, person: &str, item: &str) -> Option<Option<String>>;
}
