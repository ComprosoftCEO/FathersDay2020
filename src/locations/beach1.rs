use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const BEACH1_IMAGE: &str = r#"
                        ^                        
<-left        .        up                    .   
__      .                           `            
  \              .                      .        
  |                       .                      
  |_      `                      `        right->
   \            .                          `     
   |` .     ~~              `     .              
  `|      ~~~~~     `    ~~~~               .    
`  |  ~~~~~~~~~~~     ~~~~~~~~~~       ~~~~~   ~~
   |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

const BEACH1_LADDER: &str = r#"
                        ^                        
<-left        .        up                    .   
__   |  |                           `            
  \  |==|        .                      .        
  |  |  |                 .                      
  |_ |==| `                      `        right->
   \ |  |       .                          `     
   |`|==|   ~~              `     .              
  `| |  | ~~~~~     `    ~~~~               .    
`  | |==|~~~~~~~~     ~~~~~~~~~~       ~~~~~   ~~
   |~|~~|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach1() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Beach", get_image)
    .add_location("up", crate::locations::plains1)
    .add_location("right", crate::locations::beach2)
    .add_dynamic_location("left", climb_cliffs)
    .add_use_item("ladder", Item::Ladder)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_used_item(Item::Ladder) {
    BEACH1_LADDER.into()
  } else {
    BEACH1_IMAGE.into()
  }
}

fn climb_cliffs(state: &mut State) -> GameAction {
  if state.has_used_item(Item::Ladder) {
    GameAction::MoveTo(crate::locations::cliffs_bottom())
  } else {
    GameAction::ShowMessage(MessageType::Generic(
      "The cliffs are too steep to climb. If only you had a ladder...".into(),
    ))
  }
}
