use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const HERMIT_HUT_IMAGE: &str = r#"
        __                                       
       ()()                   \/           _____ 
 \/     ())              ___________      ((() ))
        ||      \/      /    ___    \       ( () 
                       /    /   \    \   \/  ()) 
---`-`-`-`--`---````-`/____/ hut \____\      ||  
<-left                                 `     ||  
                                        `    ||  
```-`-`--`--`--`-```-`-`------`--`-``-`-|        
                        \/                  \/   
         \/                          \/          
"#;

pub fn hermit_hut() -> Box<dyn Location> {
  LocationBuilder::new("Hermit Hut", HERMIT_HUT_IMAGE)
    .add_location("left", super::river_bridge)
    .add_location("hut", super::other::hut_inside)
    .finish()
}
