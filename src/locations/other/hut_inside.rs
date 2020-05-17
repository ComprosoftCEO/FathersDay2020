use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const HUT_INSIDE_IMAGE: &str = r#"
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

pub fn hut_inside() -> Box<dyn Location> {
  LocationBuilder::new("Hermit Hut", HUT_INSIDE_IMAGE)
    .add_location("out", crate::locations::hermit_hut)
    .add_talk_person("hermit", "Hummm......")
    .finish()
}

// TODO: Hermit Actions
