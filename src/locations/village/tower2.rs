use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const tower2_image: &str = r#"
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

pub fn tower2() -> Box<dyn Location> {}
