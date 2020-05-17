use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const farm_image: &str = r#"
                                 ____________    
        #0 0 @ 0 ! o 0 0 o O#   / [] [ ] []  \   
        #|`|`|`|`|`|`|`|`|`|#  /______________\  
        #```````````````````#    | / barn \ |    
        #0 @ @ 0 o 0 0 o . O#    |_|______|_|    
<-left  #|`|`|`|`|`|`|`|`|`|#                    
        #```````` ###########        ()   right->
        #0 o p 0 o#                  /\          
        #|`|`|`|`|# `.` -`` `        /\          
        ###########`  down   `     farmer        
                  |     V    |                   
"#;

pub fn farm() -> Box<dyn Location> {}
