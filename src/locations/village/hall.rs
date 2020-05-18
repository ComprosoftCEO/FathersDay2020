use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const HALL_IMAGE: &str = r#"
        _________________________________        
       |  _____________________________  |       
       | |   __        ()         __   | |       
       | |  |!!|       \/        |!!|  | |       
       | |  |!!|    |=======|    |!!|  | |       
       | |  |!!|    |=======|    |!!|  | |       
       | |  |==|    ||mayor||    |==|  | |       
       | |                             | |       
       | |_________           _________| |       
       |__________ |   out   | __________|       
                 | |    V    | |                 
"#;

pub fn hall() -> Box<dyn Location> {
  LocationBuilder::new("Town Hall", HALL_IMAGE)
    .add_location("out", crate::locations::village_outside)
    .add_dynamic_talk_person("mayor", talk_mayor)
    .finish()
}

fn talk_mayor(state: &mut State) -> GameAction {
  if !state.has_or_used_item(Item::OldKey) {
    state.collect_item(Item::OldKey);
    GameAction::ShowMessage(MessageType::PersonTalking("mayor".into(), "Welcome to our small village. I hear you are trying to get into the fortress. It probably won't help much, but I've saved the old key used to unlock the gate. Unfortunately, the bridge is gone, so you will need to find some other way to cross the river.\n\nThe mayor gave you an 'oldkey'.".into()))
  } else {
    GameAction::ShowMessage(MessageType::PersonTalking(
      "mayor".into(),
      "Good luck on your quest!".into(),
    ))
  }
}
