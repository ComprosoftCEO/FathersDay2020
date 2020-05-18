use crate::location::{Location, LocationBuilder};

const DUNGEON_12: &str = r#"
                                                 
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
  LocationBuilder::new("Fortress", DUNGEON_12)
    .add_location("right", super::fortress_13)
    .finish()
}
