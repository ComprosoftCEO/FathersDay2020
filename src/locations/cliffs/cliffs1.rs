use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const cliffs1_image: &str = r#"
       /                ^           |            
       |  `            up           |            
       |                       `     \           
       |  ^        ^                  |          
       |      `            `        ^  \_        
       |     )                           \       
     _/                     (            |       
    /             `               `       \      
   /      `                               |      
 _|             ^     down            )   |_____ 
/       `               V                       \
"#;

pub fn cliffs1() -> Box<dyn Location> {}
