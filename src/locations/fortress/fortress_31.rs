use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_31: &str = r#"
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
  LocationBuilder::new("Fortress", dungeon_31)
    .add_location("up", super::fortress_21)
    .add_location("down", super::fortress_41)
    .add_location("out", crate::locations::fortress_outside)
    .add_location("right", super::fortress_32)
    .finish()
}
