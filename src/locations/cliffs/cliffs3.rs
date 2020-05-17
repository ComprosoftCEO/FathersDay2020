use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const cliffs3_image: &str = r#"
                                                 
                       guru      ~~~             
   ~~~~~~~~                                      
                        ()                       
   ~~~~~~~~~~          _\/_                      
                      / \/ \        ~~~~~~~~~~~  
                    _/`     |                    
                   /       `|    ~~~~~~~~~       
          ~~~~~~  /        ~~~~~                 
                 /  ` down   \                   
                 |      V   `|                   
"#;

pub fn cliffs3() -> Box<dyn Location> {}
