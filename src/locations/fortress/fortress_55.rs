use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_55: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_55() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_55)
    .add_location("up", super::fortress_45)
    .add_location("left", super::fortress_54)
    .finish()
}
