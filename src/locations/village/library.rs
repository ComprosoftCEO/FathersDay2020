use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const LIBRARY_IMAGE: &str = r#"
 _______________________________________________ 
|  ___________________________________________  |
| |    |!!!|     | !!|     |!!!|  () |!!!|    | |
| |    |!!!| ()  |!!!|     |!!!|  \/ |!!!|    | |
| |    | !!| \/  |!!!|     |!! |  /\ |!!!| () | |
| |    |!!!| /\  |!!!|     |!!!| mark|  !| \/ | |
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
    .add_dynamic_talk_person("mark", learn_about_hermit)
    .finish()
}

fn learn_about_hermit(state: &mut State) -> GameAction {
  state.set_action(Action::LearnAboutHermit);
  GameAction::ShowMessage(MessageType::PersonTalking("mark".into(), "If you really want to activate the temple ruins, you need to talk with the hermit. He used to be the preist that ran the temple.".into()))
}
