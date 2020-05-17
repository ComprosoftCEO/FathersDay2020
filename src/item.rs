#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
  Shell(i32),
}

impl ToString for Item {
  fn to_string(&self) -> String {
    match self {
      Item::Shell(_) => format!("Shell"),
    }
  }
}
