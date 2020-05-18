use crate::location::{Location, LocationBuilder};

const DUNGEON_22: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_22() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_22)
    .add_location("left", super::fortress_21)
    .add_location("right", super::fortress_23)
    .finish()
}
