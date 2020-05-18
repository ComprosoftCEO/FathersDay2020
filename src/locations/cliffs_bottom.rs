use crate::location::{Location, LocationBuilder};
use crate::locations::helpers::collect_starfish;
use crate::state::State;

const CLIFFS_BOTTOM_IMAGE: &str = &r#"
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

const CLIFFS_NO_STARFISH: &str = r#"
                /         ^            ` \       
               /         up        `      \___   
       _______/                `              \__
      /            `                             
_____/                 _______________   `       
     \        `       /               \_______`__
      \______________/                           
                           ____________   right->
\ ___________             /            \_________
 |           \___________/             |         
 |   `       |           |     `       |      `  
"#;

pub fn cliffs_bottom() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Cliffs", get_image)
    .add_location("up", crate::locations::cliffs::cliffs1)
    .add_location("right", crate::locations::beach1)
    .add_dynamic_item("starfish", collect_starfish(2))
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_collected_starfish(2) {
    CLIFFS_NO_STARFISH.into()
  } else {
    CLIFFS_BOTTOM_IMAGE.into()
  }
}
