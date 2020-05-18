use crate::location::{Location, LocationBuilder};

const DUNGEON_34: &str = r#"
                                                 
_____________________________________________    
                                 __________  |   
 () () () () () ()              !          ! |   
 \/ \/ \/ \/ \/ \/              ! [0]  [0] ! |   
<-left                          !    \/    ! |   
 () () () () () ()              !   [==]   ! |   
 \/ \/ \/ \/ \/ \/              !__________! |   
___________________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_34() -> Box<dyn Location> {
  LocationBuilder::new("Fortress", DUNGEON_34)
    .add_location("down", super::fortress_44)
    .add_location("left", super::fortress_33)
    .finish()
}
