use crate::action::Action;
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
      r#"You wish to be spiritually ready for the temple? Very well. You must learn to understand the following sayings:

  Do not dwell in the past, do not dream of the future, concentrate the mind on the present moment.
  You only lose what you cling to.
  No one saves us but ourselves. No one can and no one may. We ourselves must walk the path.
  Three things cannot be long hidden: the sun, the moon, and the truth.
  We are what we think. All that we are arises with our thoughts. With our thoughts, we make the world.
  Peace comes from within. Do not seek it without.
  All that we are is the result of what we have thought.
  You, yourself, as much as anybody in the entire universe, deserve your love and affection.
  Health is the greatest gift, contentment the greatest wealth, faithfulness the best relationship.

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
