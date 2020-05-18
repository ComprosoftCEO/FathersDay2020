use crate::location::{Location, LocationBuilder};

const DUNGEON_32: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_32() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_32)
    .add_location("down", super::fortress_42)
    .add_location("left", super::fortress_31)
    .finish()
}
