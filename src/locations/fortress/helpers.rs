use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, MessageType};
use crate::state::State;

pub enum TrapType {
  Spikes,
  Fire,
  Cobweb,
}

impl TrapType {
  pub fn get_message(&self) -> &'static str {
    match self {
      TrapType::Spikes => {
        "Deadly spikes are blocking the path! You need some type of solid armor to disable all of the spikes..."
      }
      TrapType::Fire => "The room is filled with fire! You need some sort of shield to douse the flames...",
      TrapType::Cobweb => {
        "A spider has filled the room with cobwebs! You need something sharp to cut away the strands..."
      }
    }
  }

  pub fn clear_message(&self) -> &'static str {
    match self {
      TrapType::Spikes => "You disarmed all of the spikes",
      TrapType::Fire => "You doused all the flames with your shield",
      TrapType::Cobweb => "You cut away all of the cobwebs with your sword",
    }
  }

  pub fn item_type(&self) -> Item {
    match self {
      TrapType::Spikes => Item::Armor,
      TrapType::Fire => Item::Shield,
      TrapType::Cobweb => Item::Sword,
    }
  }
}

pub fn get_fortress_image(trap: &'static str, no_trap: &'static str, action: Action) -> impl Fn(&State) -> String {
  move |state: &State| {
    if state.has_taken_action(action) {
      no_trap.into()
    } else {
      trap.into()
    }
  }
}

pub fn test_traps_action(
  move_to: impl Fn() -> Box<dyn Location>,
  traptype: TrapType,
  action: Action,
) -> impl Fn(&mut State) -> GameAction {
  move |state: &mut State| {
    if !state.has_taken_action(action) {
      GameAction::ShowMessage(MessageType::Generic(traptype.get_message().into()))
    } else {
      GameAction::MoveTo(move_to())
    }
  }
}

pub fn clear_traps_action(traptype: TrapType, action: Action) -> impl Fn(&mut State) -> GameAction {
  move |state: &mut State| {
    if !state.has_taken_action(action) {
      state.set_action(action);
      GameAction::RedrawWithMessage(MessageType::Generic(traptype.clear_message().into()))
    } else {
      GameAction::ShowMessage(MessageType::CantUseItem(traptype.item_type().to_string()))
    }
  }
}
