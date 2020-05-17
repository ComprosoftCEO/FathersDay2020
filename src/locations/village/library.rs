use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const library: &str = r#"
 _______________________________________________ 
|  ___________________________________________  |
| |    |!!!|     | !!|     |!!!|  () |!!!|    | |
| |    |!!!| ()  |!!!|     |!!!|  \/ |!!!|    | |
| |    | !!| \/  |!!!|     |!! |  /\ |!!!| () | |
| |    |!!!| /\  |!!!|     |!!!|     |  !| \/ | |
| |    |===| pam |===|     |===|     |===| /\ | |
| |                                       tom | |
| |________________           ________________| |
|_________________ |   out   | _________________|
                 | |    V    | |                 
"#;

pub fn library() -> Box<dyn Location> {}
