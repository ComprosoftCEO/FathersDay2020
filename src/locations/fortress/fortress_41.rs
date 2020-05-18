use crate::location::{Location, LocationBuilder};

const DUNGEON_41: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
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

pub fn fortress_41() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_41)
    .add_location("up", super::fortress_31)
    .add_location("down", super::fortress_51)
    .finish()
}
