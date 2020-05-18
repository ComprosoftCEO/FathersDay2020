use crate::location::{Location, LocationBuilder};
use crate::locations::helpers::collect_starfish;
use crate::state::State;

const RIVER_BRIDGE_IMAGE: &str = r#"
     ^                 ~~~~~~~~~~~~~     ^       
    up         \/     ~~~~~~~~~~~~~~    =*=      
                       ~~~~~~~~~~       /^\      
 \/                 ~~~~~~~~~~~~      starfish   
          \/      _______________                
---`-`---`--`--  /\_____________/\ --````-`-`-`--
<-left          / / ~~~~~~~~~~~ \ \       right->
                \/ ~~~~~~~~~~~~~ \/              
--``-`-`-`-``-`--   ~~~~~~~~~~~~~  `-`-`-`---`-`-
    down             ~~~~~~~~~~~~~               
      V        \/        ~~~~~    \/         \/  
"#;

const RIVER_BRIDGE_NO_STARFISH: &str = r#"
     ^                 ~~~~~~~~~~~~~             
    up         \/     ~~~~~~~~~~~~~~    \/       
                       ~~~~~~~~~~                
 \/                 ~~~~~~~~~~~~    \/       \/  
          \/      _______________                
---`-`---`--`--  /\_____________/\ --````-`-`-`--
<-left          / / ~~~~~~~~~~~ \ \       right->
                \/ ~~~~~~~~~~~~~ \/              
--``-`-`-`-``-`--   ~~~~~~~~~~~~~  `-`-`-`---`-`-
    down             ~~~~~~~~~~~~~               
      V        \/        ~~~~~    \/         \/  
"#;

pub fn river_bridge() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("River", get_image)
    .add_location("up", super::fortress_entrance)
    .add_location("down", super::lake)
    .add_location("left", super::village_outside)
    .add_location("right", super::hermit_hut)
    .add_dynamic_item("starfish", collect_starfish(4))
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_collected_starfish(4) {
    RIVER_BRIDGE_NO_STARFISH.into()
  } else {
    RIVER_BRIDGE_IMAGE.into()
  }
}
