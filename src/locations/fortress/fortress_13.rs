use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_13: &str = r#"
                                                 
    _________________________________________    
   |    \  /   /   /\  /\   \ /  \   /  \/  /|   
___|     \/   / \ /  \/  \   /    \ /   /\ / |___
         /\  /\  /   /\   \ / \    /   /  /      
<-left  /  \/  \/ \ /  \   /   \  / \ /  /right->
___    /   /\  /\  /    \ / \   \/   /  /   / ___
   |  /   /  \/  \/ \    \   \  /\  / \/   / |   
   |_/___/___/\___\  \  / \   \/__\/__/\__/__|   
                   |  down   |                   
                   |    V    |                   
"#;

const dungeon_13_clear: &str = r#"
                                                 
    _________________________________________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_______________           _______________|   
                   |  down   |                   
                   |    V    |                   
"#;

pub fn fortress_13() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(dungeon_13, dungeon_13_clear, Action::Clear13),
  )
  .add_location("down", super::fortress_23)
  .add_dynamic_location(
    "left",
    test_traps_action(super::fortress_12, TrapType::Cobweb, Action::Clear13),
  )
  .add_dynamic_location(
    "right",
    test_traps_action(super::fortress_14, TrapType::Cobweb, Action::Clear13),
  )
  .add_dynamic_use_item("sword", clear_traps_action(TrapType::Cobweb, Action::Clear13))
  .finish()
}
