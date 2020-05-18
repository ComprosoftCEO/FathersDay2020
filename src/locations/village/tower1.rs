use crate::location::{Location, LocationBuilder};

const TOWER1_IMAGE: &str = r#"
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

pub fn tower1() -> Box<dyn Location> {
  LocationBuilder::new("Tower", TOWER1_IMAGE)
    .add_location("up", super::tower2)
    .add_location("out", crate::locations::village_outside)
    .finish()
}
