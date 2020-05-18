use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_21: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |               !!!!!!!!!!!               |   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|___
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!       
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!right->
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!    ___
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |_______________!!!!!!!!!!!_______________|   
                   |  down   |                   
                   |    V    |                   
"#;

const dungeon_21_clear: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |___
   |                                             
   |                                      right->
   |                                          ___
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_21() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(dungeon_21, dungeon_21_clear, Action::Clear21),
  )
  .add_location("down", super::fortress_31)
  .add_dynamic_location(
    "up",
    test_traps_action(super::fortress_11, TrapType::Spikes, Action::Clear21),
  )
  .add_dynamic_location(
    "right",
    test_traps_action(super::fortress_22, TrapType::Spikes, Action::Clear21),
  )
  .add_dynamic_use_item("armor", clear_traps_action(TrapType::Spikes, Action::Clear21))
  .finish()
}
