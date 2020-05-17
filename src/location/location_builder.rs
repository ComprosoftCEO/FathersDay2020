use std::collections::HashMap;

use crate::action::Action;
use crate::item::Item;
use crate::location::Location;
use crate::state::State;

type ImmutableStateFn = Box<dyn Fn(&State) -> String>;
type MutableStateFn = Box<dyn Fn(&mut State) -> String>;
type LocationFn = Box<dyn Fn(&mut State) -> Result<Box<dyn Location>, String>>;

struct Person {
  talk_to: Option<MutableStateFn>,
  give_to: HashMap<&'static str, MutableStateFn>,
}

pub struct LocationBuilder {
  get_image: ImmutableStateFn,
  locations: HashMap<&'static str, LocationFn>,
  pickup_items: HashMap<&'static str, MutableStateFn>,
  use_items: HashMap<&'static str, MutableStateFn>,
  people: HashMap<&'static str, Person>,
}

pub struct BuiltLocation(LocationBuilder);

impl LocationBuilder {
  pub fn new(get_image: impl Fn(&State) -> String + 'static) -> Self {
    LocationBuilder {
      get_image: Box::new(get_image),
      locations: HashMap::new(),
      pickup_items: HashMap::new(),
      use_items: HashMap::new(),
      people: HashMap::new(),
    }
  }

  pub fn add_location(mut self, name: &'static str, create_location: impl Fn() -> Box<dyn Location> + 'static) -> Self {
    self.locations.insert(name, Box::new(move |_| Ok(create_location())));
    self
  }

  pub fn add_dynamic_location(
    mut self,
    name: &'static str,
    func: impl Fn(&mut State) -> Result<Box<dyn Location>, String> + 'static,
  ) -> Self {
    self.locations.insert(name, Box::new(func));
    self
  }

  pub fn add_item(mut self, name: &'static str, item: Item, action: Option<Action>) -> Self {
    self.pickup_items.insert(
      name,
      Box::new(move |state: &mut State| -> String {
        state.insert_item(item);
        if let Some(a) = action {
          state.set_action(a);
        }

        format!("Collected item {}", item.to_string())
      }),
    );
    self
  }

  pub fn add_dynamic_item(
    mut self,
    name: &'static str,
    item: Item,
    func: impl Fn(&mut State) -> String + 'static,
  ) -> Self {
    self.pickup_items.insert(name, Box::new(func));
    self
  }

  pub fn add_person(mut self, person: &'static str, talk: impl Fn(&mut State) -> String + 'static) -> Self {
    match self.people.get_mut(person) {
      Some(ref mut p) => {
        p.talk_to = Some(Box::new(talk));
      }
      None => {
        self.people.insert(
          person,
          Person {
            talk_to: Some(Box::new(talk)),
            give_to: HashMap::new(),
          },
        );
      }
    }

    self
  }

  pub fn add_give_action(
    mut self,
    person: &'static str,
    item: &'static str,
    func: impl Fn(&mut State) -> String + 'static,
  ) -> Self {
    match self.people.get_mut(person) {
      Some(ref mut person) => {
        person.give_to.insert(item, Box::new(func));
      }
      None => {
        let mut map: HashMap<&str, MutableStateFn> = HashMap::new();
        map.insert(item, Box::new(func));

        self.people.insert(
          person,
          Person {
            talk_to: None,
            give_to: map,
          },
        );
      }
    }

    self
  }

  pub fn finish(self) -> impl Location {
    BuiltLocation(self)
  }

  pub fn finish_boxed(self) -> Box<impl Location> {
    Box::new(BuiltLocation(self))
  }
}

impl Location for BuiltLocation {
  fn get_image(&self, state: &State) -> String {
    (*self.0.get_image)(state)
  }

  fn move_to(&self, state: &mut State, direction: &str) -> Option<Result<Box<dyn Location>, String>> {
    self.0.locations.get(direction).map(|l| (*l)(state))
  }

  fn pickup_item(&self, state: &mut State, item: &str) -> Option<String> {
    self.0.pickup_items.get(item).map(|i| (*i)(state))
  }

  fn use_item(&self, state: &mut State, item: &str) -> Option<String> {
    self.0.use_items.get(item).map(|i| (*i)(state))
  }

  fn talk_to(&self, state: &mut State, person: &str) -> Option<String> {
    if let Some(p) = self.0.people.get(person) {
      if let Some(ref talk_to) = p.talk_to {
        return Some((*talk_to)(state));
      } else {
        return Some(format!("Cannot talk to {}", person));
      }
    }

    None
  }

  fn give_to(&self, state: &mut State, person: &str, item: &str) -> Option<Option<String>> {
    if let Some(p) = self.0.people.get(person) {
      return Some(p.give_to.get(item).map(|i| (*i)(state)));
    }

    None
  }
}
