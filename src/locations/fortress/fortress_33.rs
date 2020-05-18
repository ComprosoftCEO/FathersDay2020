use crate::action::Action;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const DUNGEON_33: &str = r#"
                                                 
    _________________________________________    
   |               _______                   |   
   |  ~~~~~~/=====/  ((@*)\))                |___
   |  ~~~~~~\====|$$$ ()()/=======               
   |   ~~~~~~    | $$$   | v^v^v^         right->
   |    ~~~~    /_\-----/_\=======            ___
   |     ~     /   \   /   \                 |   
   |_________________________________________|   
                                                 
                                                 
"#;

const DUNGEON_33_DEFEATED: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
   |                 ______                  |___
   |                |~~    |                     
   |                |~ ~~~ |              right->
   |                | card~|                  ___
   |                |~~ ~~~|                 |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_33() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Dragon Boss!!!", get_image)
    .add_location("right", super::fortress_34)
    .add_dynamic_use_item("sword", use_sword)
    .add_dynamic_item("card", read_card)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_taken_action(Action::BossDefeated) {
    DUNGEON_33_DEFEATED.into()
  } else {
    DUNGEON_33.into()
  }
}

fn use_sword(state: &mut State) -> GameAction {
  if !state.has_taken_action(Action::BossDefeated) {
    state.set_action(Action::BossDefeated);
    GameAction::RedrawWithMessage(MessageType::Generic(
      "You slew the dragon with the magical sword".into(),
    ))
  } else {
    GameAction::ShowMessage(MessageType::CantUseItem("sword".into()))
  }
}

fn read_card(state: &mut State) -> GameAction {
  if state.has_taken_action(Action::BossDefeated) {
    GameAction::WinGame
  } else {
    GameAction::ShowMessage(MessageType::NoItem("card".into()))
  }
}
