use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const cliffs2_image: &str = r#"
                 |      ^    |                   
                 |     up `  |                   
                /            |                   
               /`  .          \                  
              |         .    ) \                 
              |    `            \                
              |             `    |               
          ___/  '        (     ^ |               
         /          .             \              
        | )    `      down    .    \             
       /    .           V       `   |            
"#;

pub fn cliffs2() -> Box<dyn Location> {}
