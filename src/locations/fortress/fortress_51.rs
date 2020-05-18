use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_51: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_51() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_51)
    .add_location("up", super::fortress_41)
    .finish()
}
