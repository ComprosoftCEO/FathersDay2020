use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const TAVERN_IMAGE: &str = r#"
 _______________________________________________ 
|  ___________________________________________  |
| |   ||    ()    ###      .  ()      ()      | |
| |   ||@   \/ .  ### ()      \/   .  \/ () ##| |
| |   ||    /\   ()   \/      /\      /\ \/ ##| |
| | ()||@  amy   \/   /\    steve  ()    /\   | |
| | /\|| .       /\            ##  \/   john #| |
| | /\||@  ###  jeff      .    ##  /\      . #| |
| |___||___________           ________________| |
|_________________ |   out   | _________________|
                 | |    V    | |                 
"#;

pub fn tavern() -> Box<dyn Location> {
  LocationBuilder::new("Tavern", TAVERN_IMAGE)
    .add_location("out", crate::locations::village_outside)
    .add_talk_person("amy", "I hear that there is a fairy cave underneath the lake...")
    .add_talk_person(
      "jeff",
      "I read somewhere that fairies only come out with a magical whistle...",
    )
    .add_talk_person("john", "Loving these drinks! You should have one, too!")
    .add_dynamic_talk_person("steve", talk_starfish_person)
    .add_give_action("steve", "starfish", use_starfish)
    .finish()
}

fn talk_starfish_person(state: &mut State) -> GameAction {
  if !state.has_or_used_item(Item::Shield) {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "steve".into(),
      "I used to be a warrior! I tried to defeat the dragon in the fortress, but I couldn't. Now, I just collect starfish.\n\nHey! Tell you what! If you bring me 5 starfish to add to my collection, I'll give you my old shield. I don't need it anymore. Deal?".into()
    ));
  } else {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "steve".into(),
      "Enjoy the new shield, and thanks for the starfish!".into(),
    ));
  }
}

fn use_starfish(state: &mut State) -> GameAction {
  if state.count_collected_starfish() == 0 {
    return GameAction::ShowMessage(MessageType::NotInInventory("starfish".into()));
  }

  if state.all_starfish_collected() {
    if !state.has_or_used_item(Item::Shield) {
      state.collect_item(Item::Shield);
      return GameAction::ShowMessage(MessageType::PersonTalking(
        "steve".into(),
        "All right! Thank you for the starfish, and here is your shield. Good luck on your quest!".into(),
      ));
    } else {
      return GameAction::ShowMessage(MessageType::NotInInventory("starfish".into()));
    }
  } else {
    return GameAction::ShowMessage(MessageType::PersonTalking(
      "steve".into(),
      format!(
        "You only have {} starfish! Come back when you have collected all 5 starfish.",
        state.count_collected_starfish()
      ),
    ));
  }
}
