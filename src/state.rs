use std::collections::HashSet;

use crate::action::Action;
use crate::item::Item;

pub struct State {
  inventory: HashSet<Item>,
  actions: HashSet<Action>,
}

impl State {
  pub fn new() -> Self {
    State {
      inventory: HashSet::new(),
      actions: HashSet::new(),
    }
  }

  pub fn insert_item(&mut self, item: Item) -> bool {
    self.inventory.insert(item)
  }

  pub fn remove_item(&mut self, item: Item) -> bool {
    self.inventory.remove(&item)
  }

  pub fn has_item(&self, item: Item) -> bool {
    self.inventory.contains(&item)
  }

  pub fn set_action(&mut self, action: Action) -> bool {
    self.actions.insert(action)
  }

  pub fn has_taken_action(&self, action: Action) -> bool {
    self.actions.contains(&action)
  }

  pub fn list_inventory(&self) {
    for item in self.inventory.iter() {
      println!("{}", item.to_string());
    }
  }
}
