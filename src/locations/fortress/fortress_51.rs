use crate::location::{Location, LocationBuilder};

const DUNGEON_51: &str = r#"
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
  LocationBuilder::new("Fortress", DUNGEON_51)
    .add_location("up", super::fortress_41)
    .finish()
}
