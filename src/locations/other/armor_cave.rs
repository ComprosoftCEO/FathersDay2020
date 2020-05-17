use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const ARMOR_CAVE_IMAGE: &str = r#"
         _______________________________         
        /  ___________________________  \        
       |  /     `       __        ##  \  |       
       | |           || \/           ` | |       
       | | `         /\   ()           | |       
       | |  ##        armor    `       | |       
       | |                 ##          | |       
       | |     `                       | |       
       |  \________    out    ________/  |       
        \_________ |    V    | _________/        
                 | |         | |                 
"#;

const ARMOR_CAVE_EMPTY: &str = r#"
         _______________________________         
        /  ___________________________  \        
       |  /     `          `      ##  \  |       
       | |         ##                ` | |       
       | | `                           | |       
       | |  ##       `         `       | |       
       | |                 ##          | |       
       | |     `                       | |       
       |  \________    out    ________/  |       
        \_________ |    V    | _________/        
                 | |         | |                 
"#;

pub fn armor_cave() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Pirate Den", get_image)
    .add_location("out", crate::locations::beach4)
    .add_item("armor", Item::Armor)
    .finish()
}

fn get_image(state: &State) -> String {
  if !state.has_or_used_item(Item::Armor) {
    ARMOR_CAVE_IMAGE.into()
  } else {
    ARMOR_CAVE_EMPTY.into()
  }
}
