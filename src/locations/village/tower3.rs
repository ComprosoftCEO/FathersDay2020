use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const TOWER3_IMAGE: &str = r#"
            _________________________            
           /                         \           
          /                           \          
         /  |  |                       \         
        /   |==| ladder                 \        
       /    |  |                         \       
      |     |==|                          |      
      |     |  |                          |      
      |     |==|                          |      
      |_____|__|_______down_______________|      
     /                  V      ____|       \     
"#;

const TOWER3_NO_LADDER: &str = r#"
            _________________________            
           /                         \           
          /                           \          
         /                             \         
        /                               \        
       /                                 \       
      |                                   |      
      |                                   |      
      |                                   |      
      |________________down_______________|      
     /                  V      ____|       \     
"#;

pub fn tower3() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Tower", get_image)
    .add_location("down", super::tower2)
    .add_item("ladder", Item::Ladder)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_or_used_item(Item::Ladder) {
    TOWER3_NO_LADDER.into()
  } else {
    TOWER3_IMAGE.into()
  }
}
