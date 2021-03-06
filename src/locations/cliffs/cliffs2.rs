use crate::location::{Location, LocationBuilder};

const CLIFFS2_IMAGE: &str = r#"
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

pub fn cliffs2() -> Box<dyn Location> {
  LocationBuilder::new("Cliffs", CLIFFS2_IMAGE)
    .add_location("down", super::cliffs1)
    .add_location("up", super::cliffs3)
    .finish()
}
