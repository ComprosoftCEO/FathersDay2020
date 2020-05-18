use crate::action::Action;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const SEWER_IMAGE: &str = r#"
_______^__________________|  |__________|  |_____
       |                   \/           |  |     
_______|________| |_______~_____________|  |_____
_     out       | |                   __|  |     
 \  @           | |                  /  \  |     
  \             | |       @   valve | /\ | |     
   \__________ / /                  | \/ | |     
\   (___~____ V /                    \__/  |     
 \   |  @    \ /       ________     @   |  |     
 @|  |       | |      / ______ \        |  |     
@@|  |@@  @@@| |@@@@ / /  @   \ \       |  |@@@@@
"#;

pub fn sewer() -> Box<dyn Location> {
  LocationBuilder::new("Sewer", SEWER_IMAGE)
    .add_location("out", crate::locations::village_outside)
    .add_dynamic_use_item("valve", turn_valve)
    .finish()
}

fn turn_valve(state: &mut State) -> GameAction {
  if !state.has_taken_action(Action::SewerStopped) {
    state.set_action(Action::SewerStopped);
    GameAction::ShowMessage(MessageType::Generic(
      "You hear the water slowly draining out of the pipe...".into(),
    ))
  } else {
    GameAction::ShowMessage(MessageType::Generic("The valve is already closed!".into()))
  }
}
