use crate::location::{Location, LocationBuilder};

const VILLAGE_IMAGE: &str = r#"
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

pub fn village_outside() -> Box<dyn Location> {
  LocationBuilder::new("Village", VILLAGE_IMAGE)
    .add_location("up", super::farm)
    .add_location("down", super::plains2)
    .add_location("left", super::forest_maze)
    .add_location("right", super::river_bridge)
    .add_location("tower", super::village::tower1)
    .add_location("hall", super::village::hall)
    .add_location("library", super::village::library)
    .add_location("tavern", super::village::tavern)
    .add_location("sewer", super::village::sewer)
    .finish()
}
