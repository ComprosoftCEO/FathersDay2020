use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

pub static image: &str = &r"(
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
``````````````````````````````````````````````````
)";

pub fn new_beach() -> Box<dyn Location> {
  LocationBuilder::new(get_string)
    .add_location("beach", new_beach)
    .add_location("beach2", new_beach)
    .add_location("beach3", new_beach)
    .add_item("shell", Item::Shell(1), None)
    .finish_boxed()
}

fn get_string(s: &State) -> String {
  "String".into()
}
