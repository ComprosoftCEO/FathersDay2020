use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const hut_inside_image: &str = r#"
        _________________________________        
       |  _____________________________  |       
       | |     `       ()              | |       
       | |             \/            ` | |       
       | | `           \/              | |       
       | |           hermit    `       | |       
       | |                             | |       
       | |     `                       | |       
       | |_________    out    _________| |       
       |__________ |    V    | __________|       
                 | |         | |                 
"#;

pub fn hut_inside() -> Box<dyn Location> {}
