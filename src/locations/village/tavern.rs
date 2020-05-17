use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const TAVERN_IMAGE: &str = r#"
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

pub fn tavern() -> Box<dyn Location> {
  LocationBuilder::new("Tavern", TAVERN_IMAGE)
    .add_talk_person("amy", "I hear that there is a fairy cave underneath the lake...")
    .add_talk_person(
      "jeff",
      "I read somewhere that fairies only come out with a magical whistle...",
    )
    .add_talk_person("john", "Loving these drinks! You should have one, too!")
    .finish()
}

//TODO: Starfish Person
