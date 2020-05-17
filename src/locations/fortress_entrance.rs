use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, LocationBuilder, MessageType};
use crate::state::State;

const FORTRESS_ENTRANCE_IMAGE: &str = r#"
                        _________________        
________               /~~~          ____\_______
        \_____________/~~~~~~    \/ |[ ][_][_][_]
  \/                 ~~~~~~~~~~~    |][ ]|  \/   
                    ~~~~~~~~~~~~~   |[ ][|       
<-left               ~~~~~~~~~~~     #### right->
        \/      \/     ~~~~~~~~~~    #{}#        
                       ~~~~~~~~~     ####      \/
 \/         \/        ~~~~~~~~~~    |[ ][|  \/   
    down            ~~~~~~~~~~~~~   |][ ]|_______
      V        \/      ~~~~~~~~~~~~ |[ ][ ][ ][ ]
"#;

const FORTRESS_ENTRANCE_LOG: &str = r#"
                        _________________        
________               /~~~          ____\_______
        \_____________/~~~~~~    \/ |[ ][_][_][_]
  \/                 ~~~~~~~~~~~    |][ ]|  \/   
                    ~~~~~~~~~~~~~   |[ ][|       
<-left               ~~~~~~~~~~~     #### right->
        \/      \/ (`======`====`=)  #{}#        
                   (==``======`===)  ####      \/
 \/         \/        ~~~~~~~~~~    |[ ][|  \/   
    down            ~~~~~~~~~~~~~   |][ ]|_______
      V        \/      ~~~~~~~~~~~~ |[_][_][_][_]
"#;

const FORTRESS_ENTRANCE_OPEN: &str = r#"
                        _________________        
________               /~~~          ____\_______
        \_____________/~~~~~~    \/ |[ ][_][_][_]
  \/                 ~~~~~~~~~~~    |][ ]|  \/   
                    ~~~~~~~~~~~~~   |[ ][|       
<-left               ~~~~~~~~~~~     #  # right->
        \/      \/ (`======`====`=)              
                   (==``======`===)  #  #      \/
 \/         \/        ~~~~~~~~~~    |[ ][|  \/   
    down            ~~~~~~~~~~~~~   |][ ]|_______
      V        \/      ~~~~~~~~~~~~ |[_][_][_][_]
"#;

pub fn fortress_entrance() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Old Fortress Entrance", get_image)
    .add_location("down", super::river_bridge)
    .add_location("left", super::farm)
    .add_dynamic_location("right", test_fortress_entrance)
    .add_use_item("log", Item::Log)
    .add_dynamic_use_item("oldkey", test_old_key)
    .finish()
}

fn get_image(state: &State) -> String {
  if state.has_used_item(Item::Log) {
    if state.has_used_item(Item::OldKey) {
      FORTRESS_ENTRANCE_OPEN.into()
    } else {
      FORTRESS_ENTRANCE_LOG.into()
    }
  } else {
    FORTRESS_ENTRANCE_IMAGE.into()
  }
}

fn test_fortress_entrance(state: &mut State) -> GameAction {
  if !state.has_used_item(Item::Log) {
    return GameAction::ShowMessage(MessageType::Generic(
      "The river is too deep to cross! It's too bad we don't have a log for building a bridge...".into(),
    ));
  }

  if !state.has_used_item(Item::OldKey) {
    return GameAction::ShowMessage(MessageType::Generic(
      "The gate to the old fortress is locked! It's too bad we don't have some sort of old key to open the gate..."
        .into(),
    ));
  }

  GameAction::MoveTo(super::fortress_outside())
}

fn test_old_key(state: &mut State) -> GameAction {
  if !state.has_used_item(Item::Log) {
    return GameAction::ShowMessage(MessageType::CantUseItem("oldkey".into()));
  }

  if state.has_collected_item(Item::OldKey) {
    state.use_item(Item::OldKey);
    GameAction::RedrawWithMessage(MessageType::Generic("The old fortress gate slowly creaks open".into()))
  } else {
    GameAction::ShowMessage(MessageType::NotInInventory("oldkey".into()))
  }
}
