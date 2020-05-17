use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const tower1_image: &str = r#"
  |        ____|           ^                  | 
  |   ____|               up                  |    
  |__|________________________________________| 
 /                                       ____| \ 
|   _____                           ____|       |
|  |     |                     ____|            |
|  |     |                ____|                 |
|  |     |           ____|                      |
|  | out |      ____|                           |
|__|_____|_____|________________________________|
                                                 
"#;

pub fn tower1() -> Box<dyn Location> {}
