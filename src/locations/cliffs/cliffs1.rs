use crate::location::{Location, LocationBuilder};

const CLIFFS1_IMAGE: &str = r#"
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

pub fn cliffs1() -> Box<dyn Location> {
  LocationBuilder::new("Cliffs", CLIFFS1_IMAGE)
    .add_location("down", crate::locations::cliffs_bottom)
    .add_location("up", super::cliffs2)
    .finish()
}
