use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_53: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |___
   |                                             
   |                                      right->
   |                                          ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_53() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_53)
    .add_location("up", super::fortress_43)
    .add_location("right", super::fortress_54)
    .finish()
}
