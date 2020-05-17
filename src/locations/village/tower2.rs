use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const TOWER2_IMAGE: &str = r#"
      |                ^                  |      
      |_______________up__________________|      
     /                         ____|       \     
    /                     ____|             \    
   /                 ____|                   \   
  |             ____|                         |  
  |        ____|                              |  
  |   ____|                                   |  
  |__|________________________________________|  
 /                    down               ____| \ 
|                       V           ____|       |
"#;

pub fn tower2() -> Box<dyn Location> {
  LocationBuilder::new("Tower", TOWER2_IMAGE)
    .add_location("up", super::tower3)
    .add_location("down", super::tower1)
    .finish()
}
