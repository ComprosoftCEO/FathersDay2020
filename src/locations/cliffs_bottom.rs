use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const cliffs_bottom_image: &str = &r#"
                /         ^            ` \       
               /         up        `      \___   
       _______/                `              \__
      /            `                             
_____/                 _______________   `       
  ^  \        `       /               \_______`__
 =*=  \______________/                           
 /^\                       ____________   right->
starfish_____             /            \_________
 |           \___________/             |         
 |   `       |           |     `       |      `  
"#;

pub fn cliffs_bottom() -> Box<dyn Location> {}
