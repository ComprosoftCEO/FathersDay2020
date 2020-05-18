use crate::location::{Location, LocationBuilder};

const DUNGEON_23: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_23() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_23)
    .add_location("up", super::fortress_13)
    .add_location("left", super::fortress_22)
    .add_location("right", super::fortress_24)
    .finish()
}
