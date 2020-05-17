use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const river_bridge_image: &str = r#"
                       ~~~~~~~~~~~~~     ^       
  \/           \/     ~~~~~~~~~~~~~~    =*=      
                       ~~~~~~~~~~       /^\      
                    ~~~~~~~~~~~~      starfish   
          \/      _______________                
---`-`---`--`--  /\_____________/\ --````-`-`-`--
<-left          / / ~~~~~~~~~~~ \ \       right->
                \/ ~~~~~~~~~~~~~ \/              
--``-`-`-`-``-`--   ~~~~~~~~~~~~~  `-`-`-`---`-`-
    down             ~~~~~~~~~~~~~               
      V        \/        ~~~~~    \/         \/  
"#;

pub fn river_bridge() -> Box<dyn Location> {}
