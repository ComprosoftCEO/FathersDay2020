use crate::location::{Location, LocationBuilder};
use crate::locations::helpers::collect_starfish;
use crate::state::State;

const PLAINS3_IMAGE: &str = r#"
     ________   _______________         _________
    /        \_/               \_______/         
   /                                             
  | \/          \/    /`----`-`-`-`--`-```-`--`-`
 /       ^           ` .         `               
/       =*=         /                 .   right->
        /^\        `                             
       starfish    |  `        /-``-`-`-``--`-`-`
 \/                `        . `                  
             \/    `  down   /               \/  
    \/             |    V    |        \/         
"#;

const PLAINS3_NO_STARFISH: &str = r#"
     ________   _______________         _________
    /        \_/               \_______/         
   /                                             
  | \/          \/    /`----`-`-`-`--`-```-`--`-`
 /                   ` .         `               
/         \/        /                 .   right->
                   `                             
      \/           |  `        /-``-`-`-``--`-`-`
 \/                `        . `                  
             \/    `  down   /               \/  
    \/             |    V    |        \/         
"#;

pub fn plains3() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Grassy Plains", get_image)
    .add_location("down", super::forest_maze)
    .add_location("right", super::farm)
    .add_dynamic_item("starfish", collect_starfish(3))
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_collected_starfish(3) {
    PLAINS3_NO_STARFISH.into()
  } else {
    PLAINS3_IMAGE.into()
  }
}
