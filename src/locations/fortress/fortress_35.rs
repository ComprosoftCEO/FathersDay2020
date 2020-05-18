use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const DUNGEON_35: &str = r#"
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

const DUNGEON_35_NO_KEY: &str = r#"
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
    DUNGEON_35_NO_KEY.into()
  } else {
    DUNGEON_35.into()
  }
}
