use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_14: &str = r#"
                                                 
    _________________________________________    
   |            /\/\      /\            /\/\ |   
___|    /\    /\    /\/\     /\/\ /\/\    /\ |___
     /\    /\   /\/\    /\/\         /\/\  /\    
<-left  /\    /\    /\/\      /\  /\/\    right->
___   /\  /\    /\      /\  /\ /\     /\ /\   ___
   |    /\  /\      /\  /\       /\/\        |   
   |/\___/\____/\/\/\_/\__/\__/\___/\__/\/\__|   
                                                 
                                                 
"#;

const dungeon_14_clear: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_14() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(dungeon_14, dungeon_14_clear, Action::Clear14),
  )
  .add_location("left", super::fortress_13)
  .add_dynamic_location(
    "right",
    test_traps_action(super::fortress_15, TrapType::Fire, Action::Clear14),
  )
  .add_dynamic_use_item("shield", clear_traps_action(TrapType::Fire, Action::Clear14))
  .finish()
}
