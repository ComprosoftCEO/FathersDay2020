use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_24: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_24() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_24)
    .add_location("left", super::fortress_23)
    .finish()
}
