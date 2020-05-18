use crate::location::{Location, LocationBuilder};

const DUNGEON_24: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_24() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_24)
    .add_location("left", super::fortress_23)
    .finish()
}
