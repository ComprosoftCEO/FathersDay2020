use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::location::{Location, LocationBuilder};

const DUNGEON_43: &str = r#"
                                                 
    _________________________________________    
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
___|!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
       !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
<-left !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
___    !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |_______________!!!!!!!!!!!_______________|   
                   |  down   |                   
                   |    V    |                   
"#;

const DUNGEON_43_CLEAR: &str = r#"
                                                 
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

pub fn fortress_43() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(DUNGEON_43, DUNGEON_43_CLEAR, Action::Clear43),
  )
  .add_location("left", super::fortress_42)
  .add_dynamic_location(
    "down",
    test_traps_action(super::fortress_53, TrapType::Spikes, Action::Clear43),
  )
  .add_dynamic_use_item("armor", clear_traps_action(TrapType::Spikes, Action::Clear43))
  .finish()
}
