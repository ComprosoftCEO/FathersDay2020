use crate::location::{Location, LocationBuilder};

const DUNGEON_53: &str = r#"
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
  LocationBuilder::new("Fortress", DUNGEON_53)
    .add_location("up", super::fortress_43)
    .add_location("right", super::fortress_54)
    .finish()
}
