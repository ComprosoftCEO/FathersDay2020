use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const plains3_image: &str = r#"
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

pub fn plains3() -> Box<dyn Location> {}
