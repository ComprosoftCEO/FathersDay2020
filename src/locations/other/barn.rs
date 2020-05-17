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

const HAY1: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `     ___         `  /   \         .     `  |
| .      /   \ .        / hay \                 |
|   ___ / hay \        /       \    `    ___    |
|  /   Y       \                        /   \   |
| / hay \  .       `                   / hay \  |
|/       \                     `      /___    \ |
| `          .                     `  /   \     |
|       `          .   out   .       / hay \    |
|_____________________  V  _____________________|
"#;

const HAY2: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `     ___         `  /   \         .     `  |
| .      /   \ .        / hay \                 |
|   ___ / hay \        /       \    `    ___    |
|  /   Y       \                        /   \   |
| / hay \  .       `                   / hay \  |
|/       \                     `      /       \ |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

const HAY3: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `    `            `  /   \         .     `  |
| .            .        / hay \                 |
|   ___     .          /       \    `    ___    |
|  /   \                                /   \   |
| / hay \  .       `                   / hay \  |
|/       \                     `      /       \ |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

const HAY4: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `    `            `  /   \         .     `  |
| .            .        / hay \                 |
|   ___     .          /       \    `      `    |
|  /   \                                   .    |
| / hay \  .       `                   `        |
|/       \                     `              ` |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

const HAY5: &str = r#"
 _______________________________________________ 
|              `          ___                   |
|   `    `            `  /   \         .     `  |
| .            .        / hay \                 |
|           .          /       \    `      `    |
|  `                                       .    |
|      `   .       `                   `        |
|                              `              ` |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

const HAY6: &str = r#"
 _______________________________________________ 
|              `                `               |
|   `    `            `    |||         .     `  |
| .            .           \T/                  |
|           .            `  |       `      `    |
|  `                        |              .    |
|      `   .       `    pitchfork      `        |
|                              `              ` |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

const NO_PITCHFORK: &str = r#"
 _______________________________________________ 
|              `           `    `               |
|   `    `            `                .     `  |
| .            .                                |
|           .            `    `     `      `    |
|  `                                       .    |
|      `   .       `                   `        |
|                        `     `              ` |
| `          .                     `            |
|       `          .   out   .           `      |
|_____________________  V  _____________________|
"#;

pub fn barn() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Barn", get_image)
    .add_location("out", crate::locations::farm)
    .add_dynamic_item("hay", take_hay)
    .add_dynamic_item("pitchfork", take_pitchfork)
    .finish()
}

fn get_image(state: &State) -> String {
  match state.get_number_hay_cleaned() {
    0 => BARN_IMAGE.into(),
    1 => HAY1.into(),
    2 => HAY2.into(),
    3 => HAY3.into(),
    4 => HAY4.into(),
    5 => HAY5.into(),
    _ => {
      if state.has_or_used_item(Item::Pitchfork) {
        NO_PITCHFORK.into()
      } else {
        HAY6.into()
      }
    }
  }
}

fn take_hay(state: &mut State) -> GameAction {
  if state.is_hay_all_clean() {
    GameAction::ShowMessage(MessageType::NoItem("hay".into()))
  } else {
    state.clean_hay();
    GameAction::RedrawScreen()
  }
}

fn take_pitchfork(state: &mut State) -> GameAction {
  if state.is_hay_all_clean() && !state.has_or_used_item(Item::Pitchfork) {
    state.collect_item(Item::Pitchfork);
    GameAction::RedrawScreen()
  } else {
    GameAction::ShowMessage(MessageType::NoItem("pitchfork".into()))
  }
}
