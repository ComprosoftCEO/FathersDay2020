use super::helpers::{clear_traps_action, get_fortress_image, test_traps_action, TrapType};
use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const dungeon_54: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |    \  /   /   /\  /\   \ /  \   /  \/  /|   
___|     \/   / \ /  \/  \   /    \ /   /\ / |___
         /\  /\  /   /\   \ / \    /   /  /      
<-left  /  \/  \/ \ /  \   /   \  / \ /  /right->
___    /   /\  /\  /    \ / \   \/   /  /   / ___
   |  /   /  \/  \/ \    \   \  /\  / \/   / |   
   |_/___/___/\___\__\__/_\___\/__\/__/\__/__|   
                                                 
                                                 
"#;

const dungeon_54_clear: &str = r#"
                   |    ^    |                   
    _______________|   up    |_______________    
   |                                         |   
___|                                         |___
                                                 
<-left                                    right->
___                                           ___
   |                                         |   
   |_________________________________________|   
                                                 
                                                 
"#;

pub fn fortress_54() -> Box<dyn Location> {
  LocationBuilder::new_dynamic(
    "Fortress",
    get_fortress_image(dungeon_54, dungeon_54_clear, Action::Clear54),
  )
  .add_location("left", super::fortress_53)
  .add_dynamic_location(
    "up",
    test_traps_action(super::fortress_44, TrapType::Cobweb, Action::Clear54),
  )
  .add_dynamic_location(
    "right",
    test_traps_action(super::fortress_55, TrapType::Cobweb, Action::Clear54),
  )
  .add_dynamic_use_item("sword", clear_traps_action(TrapType::Cobweb, Action::Clear54))
  .finish()
}
