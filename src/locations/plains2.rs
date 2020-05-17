use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const plains2_image: &str = r#"
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

pub fn plains2() -> Box<dyn Location> {}
