use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_25: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |               !!!!!!!!!!!               |   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!|   
   |_______________!!!!!!!!!!!_______________|   
                   |  down   |                   
                   |    V    |                   
"#;

const dungeon_25_clear: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_25() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(dungeon_25, dungeon_25_clear, Action::Clear25),
  )
  .add_location("up", super::fortress_15)
  .add_dynamic_location(
    "down",
    test_traps_action(super::fortress_35, TrapType::Spikes, Action::Clear25),
  )
  .add_dynamic_use_item("armor", clear_traps_action(TrapType::Spikes, Action::Clear25))
  .finish()
}
