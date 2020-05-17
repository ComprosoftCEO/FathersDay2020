use crate::location::{GameAction, MessageType};
use crate::state::State;

pub fn collect_starfish(index: usize) -> impl Fn(&mut State) -> GameAction {
  move |state: &mut State| -> GameAction {
    state.collect_starfish(index);
    GameAction::RedrawWithMessage(MessageType::Generic("You found a starfish!".into()))
  }
}
