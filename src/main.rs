mod action;
mod game;
mod item;
mod location;
mod locations;
mod state;

use game::Game;

fn main() {
  let g = Game::new();
  g.run();
}
