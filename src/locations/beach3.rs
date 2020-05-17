use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const BEACH3_IMAGE: &str = r#"

      ||           | |    ___  |     |  | |      
      ||           | |   /   \ |     |  | |      
      ||           | |   | !!| |_____|  | |      
______||           | |___\!~~!_/      \ | |      
      \|___________|/     !~~!         \|_|      
<-left           `        !~~!       .    \______
   `                .     !~~! .            `    
      .      `            !~~!        `   right->
     ~~       ~  .        !~~!  ~~~~   ~~~       
 .  ~~~~~ ~  ~~~~  ~~~~~~~~~~~~~~~~~~  ~~~~   ~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

const BEACH3_NO_SEWAGE: &str = r#"
      ||           | |    ___  |     |  | |      
      ||           | |   /   \ |     |  | |      
      ||           | |   |   | |_____|  | |      
______||           | |___\___/_/      \ | |      
      \|___________|/  `     .         \|_|      
<-left           `                   .    \______
   `                .          .            `    
      .      `                        `   right->
     ~~       ~  .       .      ~~~~   ~~~       
 .  ~~~~~ ~  ~~~~  ~~~~~~~~~~~~~~~~~~  ~~~~   ~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach3() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Beach", get_image)
    .add_location("left", crate::locations::beach2)
    .add_dynamic_location("right", test_sewage)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_taken_action(Action::SewerStopped) {
    BEACH3_NO_SEWAGE.into()
  } else {
    BEACH3_IMAGE.into()
  }
}

fn test_sewage(state: &mut State) -> GameAction {
  if state.has_taken_action(Action::SewerStopped) {
    GameAction::MoveTo(crate::locations::beach4())
  } else {
    GameAction::ShowMessage(MessageType::Generic(
      "Stinky sewage is blocking your path. Maybe you can find a shutoff valve somewhere...".into(),
    ))
  }
}
