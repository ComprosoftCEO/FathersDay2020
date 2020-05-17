use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const barn_image: &str = r#"
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

pub fn barn() -> Box<dyn Location> {}
