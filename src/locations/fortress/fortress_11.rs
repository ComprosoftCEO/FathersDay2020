use crate::location::{Location, LocationBuilder};

const DUNGEON_11: &str = r#"
                                                 
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
  LocationBuilder::new("Fortress", DUNGEON_11)
    .add_location("down", super::fortress_21)
    .finish()
}
