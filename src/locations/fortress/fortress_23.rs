use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_23: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_23() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_23)
    .add_location("up", super::fortress_13)
    .add_location("left", super::fortress_22)
    .add_location("right", super::fortress_24)
    .finish()
}
