use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_11: &str = r#"
                                                 
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

pub fn fortress_11() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_11)
    .add_location("down", super::fortress_21)
    .finish()
}
