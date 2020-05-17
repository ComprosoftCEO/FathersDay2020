#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
  Ladder,
}

impl ToString for Item {
  fn to_string(&self) -> String {
    match self {
      Item::Ladder => "ladder".into(),
    }
  }
}
