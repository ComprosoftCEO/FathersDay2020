use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const FARM_IMAGE: &str = r#"
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

pub fn farm() -> Box<dyn Location> {
  LocationBuilder::new("Farm", FARM_IMAGE)
    .add_location("down", super::village_outside)
    .add_location("left", super::plains3)
    .add_location("right", super::fortress_entrance)
    .add_location("barn", super::other::barn)
    .finish()
}

// TODO: add farmer
