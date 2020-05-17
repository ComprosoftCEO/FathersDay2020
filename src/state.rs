use std::collections::{HashMap, HashSet};

use crate::action::Action;
use crate::item::Item;

pub struct State {
  inventory: HashMap<Item, ItemStatus>,
  actions: HashSet<Action>,
  starfish: [bool; 5],
}

#[derive(PartialEq)]
enum ItemStatus {
  Collected,
  Used,
}

impl State {
  pub fn new() -> Self {
    State {
      inventory: HashMap::new(),
      actions: HashSet::new(),
      starfish: [false; 5],
    }
  }

  pub fn collect_item(&mut self, item: Item) {
    self.inventory.insert(item, ItemStatus::Collected);
  }

  pub fn use_item(&mut self, item: Item) {
    self.inventory.insert(item, ItemStatus::Used);
  }

  pub fn has_collected_item(&self, item: Item) -> bool {
    self
      .inventory
      .get(&item)
      .map::<bool, _>(|s| *s == ItemStatus::Collected)
      .unwrap_or(false)
  }

  pub fn has_used_item(&self, item: Item) -> bool {
    self
      .inventory
      .get(&item)
      .map::<bool, _>(|s| *s == ItemStatus::Used)
      .unwrap_or(false)
  }

  pub fn has_or_used_item(&self, item: Item) -> bool {
    self.inventory.contains_key(&item)
  }

  pub fn set_action(&mut self, action: Action) -> bool {
    self.actions.insert(action)
  }

  pub fn has_taken_action(&self, action: Action) -> bool {
    self.actions.contains(&action)
  }

  pub fn collect_starfish(&mut self, index: usize) {
    if index > 0 && index <= 5 {
      self.starfish[index - 1] = true
    }
  }

  pub fn has_collected_starfish(&self, index: usize) -> bool {
    if index > 0 && index <= 5 {
      self.starfish[index - 1]
    } else {
      false
    }
  }

  pub fn count_collected_starfish(&self) -> u32 {
    self
      .starfish
      .iter()
      .fold(0, |acc, collected| if *collected { acc + 1 } else { acc })
  }

  pub fn list_inventory(&self) {
    // Print shells
    let num_starfish = self.count_collected_starfish();
    if num_starfish > 0 {
      println!("starfish (x{})", num_starfish);
    }

    // Print collected items
    for (item, status) in self.inventory.iter() {
      if *status == ItemStatus::Collected {
        println!("{}", item.to_string());
      }
    }
  }
}
