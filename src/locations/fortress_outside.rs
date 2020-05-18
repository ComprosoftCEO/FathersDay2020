use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const FORTRESS_OUTSIDE_IMAGE: &str = r#"
_______________________ /===\ _________________  
[_][_][_][_][_][_][_][_|() ()|[_][_][_][_][ ][ | 
       \/              |_____|            |[ ][| 
 \/                    / @_! \        \/  |][ ]| 
                \/    |# / \ %|           |[ ][| 
<-left     \/        `|__|_|__|`          |][ ]| 
                     |$/ \ / \!|   \/     |[ ][| 
       \/      \/    |_| |_| |_|       \/ |][ ]| 
______________________fortress____________|[ ][| 
[_][_][_][_][_][_][_][_][_][_][_][_][_][_][_][_| 
                                                 
"#;

pub fn fortress_outside() -> Box<dyn Location> {
  LocationBuilder::new("The Old Fortress", FORTRESS_OUTSIDE_IMAGE)
    .add_location("left", super::fortress_entrance)
    .add_dynamic_location("fortress", test_fortress)
    .finish()
}

fn test_fortress(state: &mut State) -> GameAction {
  if !(state.has_collected_item(Item::Sword)
    && state.has_collected_item(Item::Shield)
    && state.has_collected_item(Item::Armor))
  {
    GameAction::ShowMessage(MessageType::Generic(
      "You must have a sword, shield, and armor to enter the fortress...".into(),
    ))
  } else {
    GameAction::MoveTo(super::fortress::fortress_31())
  }
}
