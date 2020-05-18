use crate::location::{Location, LocationBuilder};

const DUNGEON_31: &str = r#"
                   |    ^    |                   
___________________|   up    |_______________    
||                                           |   
|| () () () ()                               |___
|| \/ \/ \/ \/                                   
<-out                                     right->
|| () () () ()                                ___
|| \/ \/ \/ \/                               |   
||_________________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_31() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_31)
    .add_location("up", super::fortress_21)
    .add_location("down", super::fortress_41)
    .add_location("out", crate::locations::fortress_outside)
    .add_location("right", super::fortress_32)
    .finish()
}
