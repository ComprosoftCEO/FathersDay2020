use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const PLAINS2_IMAGE: &str = r#"
  \/              |    ^    |     \/ _______     
         \/       `   up   .`       (   ))())    
                  /         ` \/     ()() ()     
--```-----`-`-`--/  .       `          )()       
    .          .          . |    \/    ||        
<-left                      `    ____  ||    \/  
         .                  |   ()   ) ||        
---```-`--`--`-``-\  .      `    (())  ||        
                   ``       `      ()            
            \/      ` down  |      ||      \/    
     \/             |   V   |      || \/         
"#;

pub fn plains2() -> Box<dyn Location> {
  LocationBuilder::new("Grassy Plains", PLAINS2_IMAGE)
    .add_location("up", super::village_outside)
    .add_location("down", super::beach2)
    .add_location("left", super::plains1)
    .finish()
}
