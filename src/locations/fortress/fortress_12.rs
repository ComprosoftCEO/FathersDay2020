use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_12: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
   |                                         |___
   |                                             
   |                                      right->
   |                                          ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_12() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", dungeon_12)
    .add_location("right", super::fortress_13)
    .finish()
}
