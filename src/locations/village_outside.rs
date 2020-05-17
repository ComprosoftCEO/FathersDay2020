use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const village_image: &str = r#"
      (  0  )           ^                     `  
      (     )    `     up       ============     
      (  0  )                    ||      ||   .  
  `   (tower)   .                || hall ||      
      (_||__)       _____    `   ||______||      
<-left             (sewer)                right->
                                 ----            
     _______  `    .       .    |0  0|    `      
    /_______\                  /======\          
.   |library|     `    down    |tavern|       `  
    |__||___|           V      |  ||  |  .       
"#;

pub fn village_outside() -> Box<dyn Location> {}
