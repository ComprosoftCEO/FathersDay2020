use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const FARM_IMAGE: &str = r#"
                                 ____________    
        #0 0 @ 0 ! o 0 0 o O#   / [] [ ] []  \   
        #|`|`|`|`|`|`|`|`|`|#  /______________\  
        #```````````````````#    | / barn \ |    
        #0 @ @ 0 o 0 0 o . O#    |_|______|_|    
<-left  #|`|`|`|`|`|`|`|`|`|#                    
        #```````` ###########        ()   right->
        #0 o p 0 o#                  /\          
        #|`|`|`|`|# `.` -`` `        /\          
        ###########`  down   `     farmer        
                  |     V    |                   
"#;

pub fn farm() -> Box<dyn Location> {
  LocationBuilder::new("Farm", FARM_IMAGE)
    .add_location("down", super::village_outside)
    .add_location("left", super::plains3)
    .add_location("right", super::fortress_entrance)
    .add_location("barn", super::other::barn)
    .add_dynamic_talk_person("farmer", talk_farmer)
    .add_give_action("farmer", "pitchfork", give_pitchfork)
    .finish()
}

fn talk_farmer(state: &mut State) -> GameAction {
  if state.has_or_used_item(Item::Log) {
    GameAction::ShowMessage(MessageType::PersonTalking(
      "farmer".into(),
      "Thank you so much for finding my pitchfork!".into(),
    ))
  } else {
    GameAction::ShowMessage(MessageType::PersonTalking(
      "farmer".into(),
      "Ahh! I've misplaced my pitchfork under piles of hay in the barn. Do you think you could help me?".into(),
    ))
  }
}

fn give_pitchfork(state: &mut State) -> GameAction {
  if state.can_use_item(Item::Pitchfork) {
    state.collect_item(Item::Log);
    state.use_item(Item::Pitchfork);
    GameAction::ShowMessage(MessageType::PersonTalking("farmer".into(), "You found it! Thank you so much!\nI'm a poor farmer, so I don't have much to give you in return. But maybe you can use this log I chopped up myself.\n\nThe farmer gave you 'log'.".into()))
  } else {
    GameAction::ShowMessage(MessageType::NotInInventory("pitchfork".into()))
  }
}
