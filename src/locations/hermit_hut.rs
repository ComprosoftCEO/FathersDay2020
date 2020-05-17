use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const hermit_hut_image: &str = r#"
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

pub fn hermit_hut() -> Box<dyn Location> {}
