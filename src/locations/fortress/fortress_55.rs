use crate::location::{Location, LocationBuilder};

const DUNGEON_55: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
___|                                         |   
                                             |   
<-left                                       |   
___                                          |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_55() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_55)
    .add_location("up", super::fortress_45)
    .add_location("left", super::fortress_54)
    .finish()
}
