use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::location::{Location, LocationBuilder};

const DUNGEON_42: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |            /\/\      /\            /\/\ |   
   |    /\    /\    /\/\     /\/\ /\/\    /\ |___
   | /\    /\   /\/\    /\/\         /\/\  /\    
   |  /\    /\    /\/\      /\  /\/\      right->
   |   /\  /\    /\      /\  /\ /\     /\ /\  ___
   |    /\  /\      /\  /\       /\/\        |   
   |/\___/\____/\/\/\ /\  /\  /\___/\__/\/\__|   
                   |  down   |                   
                   |    V    |                   
"#;

const DUNGEON_42_CLEAR: &str = r#"
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

pub fn fortress_42() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(DUNGEON_42, DUNGEON_42_CLEAR, Action::Clear42),
  )
  .add_location("up", super::fortress_32)
  .add_dynamic_location(
    "down",
    test_traps_action(super::fortress_52, TrapType::Fire, Action::Clear42),
  )
  .add_dynamic_location(
    "right",
    test_traps_action(super::fortress_43, TrapType::Fire, Action::Clear42),
  )
  .add_dynamic_use_item("shield", clear_traps_action(TrapType::Fire, Action::Clear42))
  .finish()
}
