use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const beach4_image: &str = r#"
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

pub fn beach4() -> Box<dyn Location> {}
