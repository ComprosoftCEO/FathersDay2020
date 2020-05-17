use crate::location::GameAction;
use crate::state::State;

pub trait Location {
  fn get_image(&self, state: &State) -> String;

  // Interactions
  fn move_to(&self, state: &mut State, direction: &str) -> GameAction;
  fn pickup_item(&self, state: &mut State, item: &str) -> GameAction;
  fn use_item(&self, state: &mut State, item: &str) -> GameAction;
  fn talk_to(&self, state: &mut State, person: &str) -> GameAction;
  fn give_to(&self, state: &mut State, person: &str, item: &str) -> GameAction;
}
