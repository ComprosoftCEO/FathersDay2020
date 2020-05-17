use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const BARN_IMAGE: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `     ___         `  /   \         .     `  |
| .      /   \ .        / hay \                 |
|   ___ / hay \        /       \    `    ___    |
|  /   Y       \                        /   \   |
| / hay \  .       `                   / hay \  |
|/       \        ___          `      /___    \ |
| `          .   /hay\             `  /   \     |
|       `              out   .       / hay \    |
|_____________________  V  _____________________|
"#;

pub fn barn() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Barn", get_image)
    .add_location("out", crate::locations::farm)
    .add_dynamic_item("hay", take_hay)
    .finish()
}

// TODO: Hay Actions!

fn get_image(_state: &State) -> String {
  BARN_IMAGE.into()
}

fn take_hay(_state: &mut State) -> GameAction {
  GameAction::RedrawScreen()
}
