use crate::location::{Location, LocationBuilder};

const DUNGEON_45: &str = r#"
                                                 
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
  LocationBuilder::new("Fortress", DUNGEON_45)
    .add_location("down", super::fortress_55)
    .finish()
}
