use crate::location::{Location, LocationBuilder};

const DUNGEON_15: &str = r#"
                                                 
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
pub fn fortress_15() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_15)
    .add_location("down", super::fortress_25)
    .add_location("left", super::fortress_14)
    .finish()
}
