use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const armor_cave: &str = r#"
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

pub fn armor_cave() -> Box<dyn Location> {}
