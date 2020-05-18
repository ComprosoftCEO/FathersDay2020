use std::collections::HashMap;

use crate::action::Action;
use crate::item::Item;
use crate::location::{GameAction, Location, MessageType};
use crate::state::State;

type BoxedLocationFn = Box<dyn Fn(&State) -> String>;
type BoxedStateFn = Box<dyn Fn(&mut State) -> GameAction>;

struct Person {
  talk_to: Option<BoxedStateFn>,
  give_to: HashMap<&'static str, BoxedStateFn>,
}

enum ImageType {
  Static(&'static str),
  Dynamic(BoxedLocationFn),
}

pub struct LocationBuilder {
  label: &'static str,
  image: ImageType,
  locations: HashMap<&'static str, BoxedStateFn>,
  pickup_items: HashMap<&'static str, BoxedStateFn>,
  use_items: HashMap<&'static str, BoxedStateFn>,
  people: HashMap<&'static str, Person>,
}

pub struct BuiltLocation(LocationBuilder);

impl LocationBuilder {
  pub fn new(label: &'static str, image: &'static str) -> Self {
    LocationBuilder {
      label,
      image: ImageType::Static(image),
      locations: HashMap::new(),
      pickup_items: HashMap::new(),
      use_items: HashMap::new(),
      people: HashMap::new(),
    }
  }

  pub fn new_dynamic(label: &'static str, get_image: impl Fn(&State) -> String + 'static) -> Self {
    LocationBuilder {
      label,
      image: ImageType::Dynamic(Box::new(get_image)),
      locations: HashMap::new(),
      pickup_items: HashMap::new(),
      use_items: HashMap::new(),
      people: HashMap::new(),
    }
  }

  pub fn add_location(mut self, name: &'static str, create_location: impl Fn() -> Box<dyn Location> + 'static) -> Self {
    self
      .locations
      .insert(name, Box::new(move |_| GameAction::MoveTo(create_location())));
    self
  }

  pub fn add_location_no_duplicate(
    self,
    name: &'static str,
    create_location: impl Fn() -> Box<dyn Location> + 'static,
  ) -> Self {
    if !self.locations.contains_key(name) {
      self.add_location(name, create_location)
    } else {
      self
    }
  }

  pub fn add_dynamic_location(mut self, name: &'static str, func: impl Fn(&mut State) -> GameAction + 'static) -> Self {
    self.locations.insert(name, Box::new(func));
    self
  }

  pub fn add_item(mut self, name: &'static str, item: Item) -> Self {
    self.pickup_items.insert(
      name,
      Box::new(move |state: &mut State| -> GameAction {
        if !state.has_or_used_item(item) {
          state.collect_item(item);
          GameAction::RedrawWithMessage(MessageType::ItemCollected(item))
        } else {
          GameAction::ShowMessage(MessageType::NoItem(name.into()))
        }
      }),
    );

    self
  }

  pub fn add_dynamic_item(mut self, name: &'static str, func: impl Fn(&mut State) -> GameAction + 'static) -> Self {
    self.pickup_items.insert(name, Box::new(func));
    self
  }

  pub fn add_use_item(mut self, name: &'static str, item: Item) -> Self {
    self.use_items.insert(
      name,
      Box::new(move |state: &mut State| -> GameAction {
        if state.can_use_item(item) {
          state.use_item(item);
          GameAction::RedrawScreen()
        } else {
          GameAction::ShowMessage(MessageType::NotInInventory(name.into()))
        }
      }),
    );

    self
  }

  pub fn add_dynamic_use_item(mut self, name: &'static str, func: impl Fn(&mut State) -> GameAction + 'static) -> Self {
    self.use_items.insert(name, Box::new(func));
    self
  }

  pub fn add_use_item_action(self, name: &'static str, action: Action) -> Self {
    self.add_dynamic_use_item(name, move |state: &mut State| -> GameAction {
      state.set_action(action);
      GameAction::RedrawScreen()
    })
  }

  pub fn add_talk_person(mut self, person: &'static str, message: &'static str) -> Self {
    self.add_dynamic_talk_person(person, move |_| {
      GameAction::ShowMessage(MessageType::PersonTalking(person.into(), message.into()))
    })
  }

  pub fn add_dynamic_talk_person(
    mut self,
    person: &'static str,
    talk: impl Fn(&mut State) -> GameAction + 'static,
  ) -> Self {
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
    func: impl Fn(&mut State) -> GameAction + 'static,
  ) -> Self {
    match self.people.get_mut(person) {
      Some(ref mut person) => {
        person.give_to.insert(item, Box::new(func));
      }
      None => {
        let mut map: HashMap<&str, BoxedStateFn> = HashMap::new();
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

  pub fn finish(self) -> Box<impl Location> {
    Box::new(BuiltLocation(self))
  }
}

impl BuiltLocation {
  pub fn center_label(&self) -> String {
    let len = (49 - self.0.label.len()) / 2;
    return " ".repeat(len) + self.0.label;
  }
}

impl Location for BuiltLocation {
  fn get_image(&self, state: &State) -> String {
    let picture: String = match &self.0.image {
      ImageType::Static(img) => (*img).into(),
      ImageType::Dynamic(func) => func(state),
    };

    format!("{}\n{}\n", picture, self.center_label())
  }

  fn move_to(&self, state: &mut State, direction: &str) -> GameAction {
    self
      .0
      .locations
      .get(direction)
      .map(|action| (*action)(state))
      .unwrap_or_else(|| GameAction::ShowMessage(MessageType::NoLocation(direction.into())))
  }

  fn pickup_item(&self, state: &mut State, item: &str) -> GameAction {
    self
      .0
      .pickup_items
      .get(item)
      .map(|action| (*action)(state))
      .unwrap_or_else(|| GameAction::ShowMessage(MessageType::NoItem(item.into())))
  }

  fn use_item(&self, state: &mut State, item: &str) -> GameAction {
    self
      .0
      .use_items
      .get(item)
      .map(|action| (*action)(state))
      .unwrap_or_else(|| match Item::find_string(item) {
        Some(i) if state.has_collected_item(i) => GameAction::ShowMessage(MessageType::CantUseItem(item.into())),
        _ => GameAction::ShowMessage(MessageType::NotInInventory(item.into())),
      })
  }

  fn talk_to(&self, state: &mut State, person: &str) -> GameAction {
    match self.0.people.get(person) {
      Some(p) => match &p.talk_to {
        Some(action) => (*action)(state),
        None => GameAction::ShowMessage(MessageType::CantTalkTo(person.into())),
      },
      None => GameAction::ShowMessage(MessageType::NoPerson(person.into())),
    }
  }

  fn give_to(&self, state: &mut State, person: &str, item: &str) -> GameAction {
    match self.0.people.get(person) {
      Some(p) => match p.give_to.get(item) {
        Some(action) => (*action)(state),
        None => match Item::find_string(item) {
          Some(i) if state.has_collected_item(i) => {
            GameAction::ShowMessage(MessageType::CantGiveItem(person.into(), item.into()))
          }
          _ => GameAction::ShowMessage(MessageType::NotInInventory(item.into())),
        },
      },
      None => GameAction::ShowMessage(MessageType::NoPerson(person.into())),
    }
  }
}
