use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_34: &str = r#"
                                                 
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
  LocationBuilder::new("Fortress", dungeon_34)
    .add_location("down", super::fortress_44)
    .add_location("left", super::fortress_33)
    .finish()
}
