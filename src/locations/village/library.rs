use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const LIBRARY_IMAGE: &str = r#"
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

pub fn library() -> Box<dyn Location> {
  LocationBuilder::new("Library", LIBRARY_IMAGE)
    .add_location("out", crate::locations::village_outside)
    .add_talk_person("pam", "I'm just looking for some books...")
    .add_talk_person("tom", "Let me tell you a secret: I found out how to get through the forest! There are tempe ruins on the other side!!! The secret is:\n\nleft up left down left left\n\nCan you remember that?")
    .finish()
}
