use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_35: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |   
   |                 bosskey                 |   
   |                                         |   
   |                ()=======                |   
   |                      V v                |   
   |_________________________________________|   
                                                 
                                                 
"#;

const dungeon_35_no_key: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_35() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Fortress", get_image)
    .add_location("up", super::fortress_25)
    .add_item("bosskey", Item::BossKey)
    .finish()
}

pub fn get_image(state: &State) -> String {
  if state.has_or_used_item(Item::BossKey) {
    dungeon_35_no_key.into()
  } else {
    dungeon_35.into()
  }
}
