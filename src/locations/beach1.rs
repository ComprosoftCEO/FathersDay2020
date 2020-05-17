use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const beach1_image: &str = &r#"
                        ^                        
<-left        .        up                    .   
__      .                           `            
  \              .                      .        
  |                       .                      
  |_      `                      `        right->
   \            .                          `     
   |` .     ~~              `     .              
  `|      ~~~~~     `    ~~~~               .    
`  |  ~~~~~~~~~~~     ~~~~~~~~~~       ~~~~~   ~~
   |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

const beach1_ladder: &str = r#"
                        ^                        
<-left        .        up                    .   
__   |  |                           `            
  \  |==|        .                      .        
  |  |  |                 .                      
  |_ |==| `                      `        right->
   \ |  |       .                          `     
   |`|==|   ~~              `     .              
  `| |  | ~~~~~     `    ~~~~               .    
`  | |==|~~~~~~~~     ~~~~~~~~~~       ~~~~~   ~~
   |~|~~|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach1() -> Box<dyn Location> {
  LocationBuilder::new(get_string)
    .add_location("beach", new_beach)
    .add_location("beach2", new_beach)
    .add_location("beach3", new_beach)
    .add_item("shell", Item::Shell(1), None)
    .finish_boxed()
}

fn get_string(s: &State) -> String {
  "String".into()
}
