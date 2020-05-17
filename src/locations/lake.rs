use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const LAKE_IMAGE: &str = r#"
      ^                  ~~~~~                   
      up        \/       ~~~~~    \/             
 \/                     ~~~~~           \/       
        \/      ~~~~~~~~~~~~~~~~~~~~         \/  
             ~~~~~~~~~~~~~~~~~~~~~~~~~~~         
     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ ~~~~~~     
        ~~~~~~~~~~~~~ ~~~~~~~~~~~                
  \/                  \/        ______           
______        \/     __________/     |\     \/   
     |\_____________/|         |     | \__       
     | |           | |         |     |  | \______
"#;

const LAKE_FAIRY: &str = r#"
      ^                  ~~~~~                   
     up         \/   @@  ~~~~~    \/             
 \/                     ~~~~~           \/       
        \/      ~~~~~~~__~~~~~~~~~~~         \/  
             ~~~~~@@  |##|      ~~~~~~~~         
     ~~~~~~~~         |##| @@       ~~ ~~~~~~    
        ~~~~~~~~~~~   cave  ~~~~~                
  \/                            ______           
______        \/     __________/     |\     \/   
     |\_____________/|         |     | \__       
     | |           | |         |     |  | \______
"#;

pub fn lake() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Lake", get_image)
    .add_location("up", super::river_bridge)
    .add_dynamic_location("cave", test_cave)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_used_item(Item::Whistle) {
    LAKE_FAIRY.into()
  } else {
    LAKE_IMAGE.into()
  }
}

fn test_cave(state: &mut State) -> GameAction {
  if state.has_used_item(Item::Whistle) {
    GameAction::MoveTo(super::other::fairy_cave())
  } else {
    GameAction::ShowMessage(MessageType::NoLocation("cave".into()))
  }
}
