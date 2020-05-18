use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_15: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;
pub fn fortress_15() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_15)
    .add_location("down", super::fortress_25)
    .add_location("left", super::fortress_14)
    .finish()
}
