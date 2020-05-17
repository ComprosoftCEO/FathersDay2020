use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const hall_image: &str = r#"
        _________________________________        
       |  _____________________________  |       
       | |   __        ()         __   | |       
       | |  |!!|       \/        |!!|  | |       
       | |  |!!|    |=======|    |!!|  | |       
       | |  |!!|    |=======|    |!!|  | |       
       | |  |==|    ||mayor||    |==|  | |       
       | |                             | |       
       | |_________           _________| |       
       |__________ |   out   | __________|       
                 | |    V    | |                 
"#;

pub fn hall() -> Box<dyn Location> {}
