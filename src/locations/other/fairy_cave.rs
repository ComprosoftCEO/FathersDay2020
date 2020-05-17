use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const FAIRY_CAVE_IMAGE: &str = r#"
        ______/      \       /\     /     \      
       /  ________.__...________.._____    \     
      /  |  @@      _______            `    |    
     |   `         /~~=~=~~\        ^  |   /     
     |   |        |~~~~V~~~~|  @@   |  `   \     
      \  ` @      |~~\~|~/~~|       |  |    |    
      /  |        |~~~( )~~~|      out |    |    
     /   `     @@  \_fairy_/   @       `   /     
     |   ._.._._____..__.__...__._____.|  /      
     \___             __________    /   \/       
         \___________/          \/\/             
"#;

pub fn fairy_cave() -> Box<dyn Location> {
  LocationBuilder::new("Fairy Cave", FAIRY_CAVE_IMAGE)
    .add_location("out", crate::locations::lake)
    .add_dynamic_talk_person("fairy", talk_fairy)
    .finish()
}

fn talk_fairy(state: &mut State) -> GameAction {
  if !state.has_or_used_item(Item::OldKey) {
    state.collect_item(Item::OldKey);
    GameAction::ShowMessage(MessageType::Generic(
      "Use this sword to defeat the evil dragon.\n\nThe fairy queen gave you 'sword'.".into(),
    ))
  } else {
    GameAction::ShowMessage(MessageType::Generic("Use the sword wisely!".into()))
  }
}
