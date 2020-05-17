use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const CLIFFS3_IMAGE: &str = r#"
                                                 
                       guru      ~~~             
   ~~~~~~~~                                      
                        ()                       
   ~~~~~~~~~~          _\/_                      
                      / \/ \        ~~~~~~~~~~~  
                    _/`     |                    
                   /       `|    ~~~~~~~~~       
          ~~~~~~  /        ~~~~~                 
                 /  ` down   \                   
                 |      V   `|                   
"#;

pub fn cliffs3() -> Box<dyn Location> {
  LocationBuilder::new("Cliffs", CLIFFS3_IMAGE)
    .add_location("down", super::cliffs2)
    .add_dynamic_talk_person("guru", talk_guru)
    .finish()
}

fn talk_guru(state: &mut State) -> GameAction {
  if !state.has_taken_action(Action::TalkedHermit) {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "guru".into(),
      "Inner peace...... Enlightenment..... Truth......".into(),
    ));
  }

  if !state.has_taken_action(Action::TalkedGuru) {
    state.set_action(Action::TalkedGuru);
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "guru".into(),
      r#"You wish to be spiritually ready for the temple? Very well. You must learn to understand the following saying:

Do not dwell in the past, do not dream of the future, concentrate the mind on the present moment.
You only lose what you cling to.
No one saves us but ourselves. No one can and no one may. We ourselves must walk the path.

Now you are ready for the temple ruins! Go!"#
        .into(),
    ));
  } else {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "guru".into(),
      "Go! You are ready for the temple.......\n\nInner peace..... truth...... enlightenment.....".into(),
    ));
  }
}
