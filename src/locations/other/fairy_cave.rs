use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const fairy_cave_image: &str = r#"
        ______/      \       /\     /     \      
       /  ________.__...________.._____    \     
      /  |  @@      _______            `    |    
     |   `         /~~=~=~~\        ^  |   /     
     |   |        |~~~~V~~~~|  @@   |  `   \     
      \  ` @      |~~\~|~/~~|       |  |    |    
      /  |        |~~~( )~~~|      out |    |    
     /   `     @@  \_fairy_/   @       `   /     
     |   ._.._._____..__.__...__._____.|  /      
     \___             __________    /   \/       
         \___________/          \/\/             
"#;

pub fn fairy_cave() -> Box<dyn Location> {}
