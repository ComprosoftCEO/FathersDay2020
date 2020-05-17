use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::locations::helpers::collect_starfish;
use crate::state::State;

const BEACH4_IMAGE: &str = r#"
      | |     |   |                        |    |
      | |     |   |      ____         ____ |    |
      | |     |   |     \    \       /    /|    |
      | |     |   |      \____\     /____/ |    |
      | |_____|   |           ______       |   /~
______|/   ^   \  |          /      \      |  /~~
      `   =*=   \ |          | cave |      | /~~~
<-left    /^\    \|__________|______|______|/~~~~
~  .    starfish        `             .    ~~~~~~
~~   ~~~~~~~ ~~~   ~~~~~  ~  ~    ~~~~~ `   ~~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

const BEACH4_NO_STARFISH: &str = r#"
      | |     |   |                        |    |
      | |     |   |      ____         ____ |    |
      | |     |   |     \    \       /    /|    |
      | |     |   |      \____\     /____/ |    |
      | |_____|   |           ______       |   /~
______|/       \  |          /      \      |  /~~
      `       ` \ |          | cave |      | /~~~
<-left           \|__________|______|______|/~~~~
~  .      `             `             .    ~~~~~~
~~   ~~~~~~~ ~~~   ~~~~~  ~  ~    ~~~~~ `   ~~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach4() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Pirate's Cove", get_image)
    .add_location("left", crate::locations::beach3)
    .add_location("cave", crate::locations::other::armor_cave)
    .add_dynamic_item("starfish", collect_starfish(5))
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_collected_starfish(5) {
    BEACH4_NO_STARFISH.into()
  } else {
    BEACH4_IMAGE.into()
  }
}
