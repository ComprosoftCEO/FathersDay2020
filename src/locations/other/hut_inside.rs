use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const HUT_INSIDE_IMAGE: &str = r#"
        _________________________________        
       |  _____________________________  |       
       | |     `       ()              | |       
       | |             \/            ` | |       
       | | `           \/              | |       
       | |           hermit    `       | |       
       | |                             | |       
       | |     `                       | |       
       | |_________    out    _________| |       
       |__________ |    V    | __________|       
                 | |         | |                 
"#;

pub fn hut_inside() -> Box<dyn Location> {
  LocationBuilder::new("Hermit Hut", HUT_INSIDE_IMAGE)
    .add_location("out", crate::locations::hermit_hut)
    .add_dynamic_talk_person("hermit", talk_hermit)
    .finish()
}

fn talk_hermit(state: &mut State) -> GameAction {
  if !state.has_taken_action(Action::LearnAboutHermit) {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "hermit".into(),
      "Hummmmm........ Hummmmm....... Don't bother me, I'm meditating! Hummmmmmm....".into(),
    ));
  }

  if !state.has_taken_action(Action::TalkedHermit) {
    state.set_action(Action::TalkedHermit);
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "hermit".into(),
      "You want to activate the temple ruins, right? Okay, but first go talk to my brother, the guru on the cliffs. He will prepare you spiritually to enter the temple...".into()
    ));
  }

  if !state.has_taken_action(Action::TalkedGuru) {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "hermit".into(),
      "Go! Talk with the guru on the cliffs. You need to be spiritually ready for the temple! Then I will help you..."
        .into(),
    ));
  }

  if !state.has_or_used_item(Item::Amulet) {
    state.collect_item(Item::Amulet);
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "hermit".into(),
      "Now you understand the path to enlightenment. You are ready to activate the temple ruins! Here is the ancient amulet.\n\nHermit gave you item 'amulet'.".into()
    ));
  } else {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "hermit".into(),
      "Hummmmmm..... Hummmmmmm..... Hummmmm.......".into(),
    ));
  }
}
