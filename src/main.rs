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
  let mut state = State::new();

  state.insert_item(Item::Shell(1));

  println!("{}", state.has_item(Item::Shell(2)));
  state.insert_item(Item::Shell(2));

  println!("{}", state.has_item(Item::Shell(2)));
}
