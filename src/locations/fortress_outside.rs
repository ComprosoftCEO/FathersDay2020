use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const FORTRESS_OUTSIDE_IMAGE: &str = r#"
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

pub fn fortress_outside() -> Box<dyn Location> {
  LocationBuilder::new("The Old Fortress", FORTRESS_OUTSIDE_IMAGE)
    .add_location("left", super::fortress_entrance)
    .finish()
}

// TODO: Add fortress
