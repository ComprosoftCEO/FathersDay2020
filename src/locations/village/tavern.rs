use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const tavern_image: &str = r#"
 _______________________________________________ 
|  ___________________________________________  |
| |   ||    ()    ###      .  ()      ()      | |
| |   ||@   \/ .  ### ()      \/   .  \/ () ##| |
| |   ||    /\   ()   \/      /\      /\ \/ ##| |
| | ()||@  amy   \/   /\    steve  ()    /\   | |
| | /\|| .       /\            ##  \/   john #| |
| | /\||@  ###  jeff      .    ##  /\      . #| |
| |___||___________           ________________| |
|_________________ |   out   | _________________|
                 | |    V    | |                 
"#;

pub fn tavern() -> Box<dyn Location> {}
