use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const TEMPLE_RUINS_IMAGE: &str = r#"
     /    _      _     \                   (   ()
    /    | |    | |     \       \/        (()    
   /     |%|    |$|      \               (    () 
  /  _   |_|    |_|   _   \  \/          (______(
 |  | |              | |  |         \/           
 |  |&|      ()      |#|  |               right->
 |  |_|   _      _   |_|  |               _______
 \       | |    | |      /             (((       
  \      |!|    |@|     /             (    ()    
   \     |_|    |_|    /       \/    (        () 
    \                 /  \/         (    ()      
"#;

const TEMPLE_RUINS_WHISTLE: &str = r#"
     /    _      _     \                   (   ()
    /    | |    | |     \       \/        (()    
   /     |%|    |$|      \               (    () 
  /  _   |_|    |_|   _   \  \/          (______(
 |  | |              | |  |         \/           
 |  |&|     ==>      |#|  |               right->
 |  |_|   whistle_   |_|  |               _______
 \       | |    | |      /             (((       
  \      |!|    |@|     /             (    ()    
   \     |_|    |_|    /       \/    (        () 
    \                 /  \/         (    ()      
"#;

pub fn temple_ruins() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Temple Ruins", get_image)
    .add_location("right", super::forest_maze)
    .add_dynamic_use_item("amulet", use_amulet)
    .add_dynamic_item("whistle", test_whistle)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_used_item(Item::Amulet) {
    if state.has_or_used_item(Item::Whistle) {
      TEMPLE_RUINS_IMAGE.into()
    } else {
      TEMPLE_RUINS_WHISTLE.into()
    }
  } else {
    TEMPLE_RUINS_IMAGE.into()
  }
}

fn use_amulet(state: &mut State) -> GameAction {
  if !state.has_used_item(Item::Amulet) {
    state.use_item(Item::Amulet);
    GameAction::RedrawWithMessage(MessageType::Generic(
      "You feel the magic stirring in the temple ruins as you use the amulet...".into(),
    ))
  } else {
    GameAction::ShowMessage(MessageType::NotInInventory("amulet".into()))
  }
}

fn test_whistle(state: &mut State) -> GameAction {
  if state.has_used_item(Item::Amulet) {
    if !state.has_or_used_item(Item::Whistle) {
      state.collect_item(Item::Whistle);
      return GameAction::RedrawWithMessage(MessageType::ItemCollected(Item::Whistle));
    }
  }

  GameAction::ShowMessage(MessageType::NoItem("whistle".into()))
}
