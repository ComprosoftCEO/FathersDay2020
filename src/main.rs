#[macro_use]
extern crate lazy_static;

mod action;
mod item;
mod location;
mod locations;
mod state;

use item::Item;
use state::State;

fn main() {
  let l = locations::beach1();
}
