use crate::location::{Location, LocationBuilder};

const DUNGEON_52: &str = r#"
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

pub fn fortress_52() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_52)
    .add_location("up", super::fortress_42)
    .finish()
}
