use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const sewer_image: &str = r#"
_______^__________________|  |__________|  |_____
       |                   \/           |  |     
_______|________| |_______~_____________|  |_____
_     out       | |                   __|  |     
 \  @           | |                  /  \  |     
  \             | |       @   valve | /\ | |     
   \__________ / /                  | \/ | |     
\   (___~____ V /                    \__/  |     
 \   |  @    \ /       ________     @   |  |     
 @|  |       | |      / ______ \        |  |     
@@|  |@@  @@@| |@@@@ / /  @   \ \       |  |@@@@@
"#;

pub fn sewer() -> Box<dyn Location> {}
