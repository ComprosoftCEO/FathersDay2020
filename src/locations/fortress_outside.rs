use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const fortress_outside_image: &str = r#"
_______________________ /===\ _________________  
[_][_][_][_][_][_][_][_|() ()|[_][_][_][_][ ][ | 
       \/              |_____|            |[ ][| 
 \/                    / @_! \        \/  |][ ]| 
                \/    |# / \ %|           |[ ][| 
<-left     \/        `|__|_|__|`          |][ ]| 
                     |$/ \ / \!|   \/     |[ ][| 
       \/      \/    |_| |_| |_|       \/ |][ ]| 
______________________fortress____________|[ ][| 
[_][_][_][_][_][_][_][_][_][_][_][_][_][_][_][_| 
                                                 
"#;

pub fn fortress_outside() -> Box<dyn Location> {}
