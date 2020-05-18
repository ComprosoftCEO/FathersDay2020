use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_45: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_45() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_45)
    .add_location("down", super::fortress_55)
    .finish()
}
