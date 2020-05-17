use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const CLIFFS3_IMAGE: &str = r#"
                                                 
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

pub fn cliffs3() -> Box<dyn Location> {
  LocationBuilder::new("Cliffs", CLIFFS3_IMAGE)
    .add_location("down", super::cliffs2)
    .finish()
}

// TODO: Program the Guru
