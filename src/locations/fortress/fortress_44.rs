use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const DUNGEON_44: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |             |=============|             |   
   |  ()   ()    |=============|    ()   ()  |   
   |  \/   \/    |=====( )=====|    \/   \/  |   
   |                                         |   
   |  ()   ()                       ()   ()  |   
   |  \/   \/                       \/   \/  |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

const DUNGEON_44_OPEN: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |             |=|         |=|             |   
   |  ()   ()    |=|         |=|    ()   ()  |   
   |  \/   \/    |=|         |=|    \/   \/  |   
   |                                         |   
   |  ()   ()                       ()   ()  |   
   |  \/   \/                       \/   \/  |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_44() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Fortress", get_image)
    .add_location("down", super::fortress_54)
    .add_dynamic_location("up", boss_door)
    .add_dynamic_use_item("bosskey", open_door)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_used_item(Item::BossKey) {
    DUNGEON_44_OPEN.into()
  } else {
    DUNGEON_44.into()
  }
}

fn open_door(state: &mut State) -> GameAction {
  if state.can_use_item(Item::BossKey) {
    state.use_item(Item::BossKey);
    GameAction::RedrawWithMessage(MessageType::Generic("The massive door slowly creaks open...".into()))
  } else {
    if state.has_collected_item(Item::BossKey) {
      GameAction::ShowMessage(MessageType::CantUseItem(Item::BossKey.to_string()))
    } else {
      GameAction::ShowMessage(MessageType::NotInInventory(Item::BossKey.to_string()))
    }
  }
}

fn boss_door(state: &mut State) -> GameAction {
  if !state.has_used_item(Item::BossKey) {
    GameAction::ShowMessage(MessageType::Generic(
      "The massive door is locked. It's too bad you don't have some sort of a 'boss key'...".into(),
    ))
  } else {
    GameAction::MoveTo(super::fortress_34())
  }
}
