use crate::location::Location;
use crate::location::MessageType;

pub enum GameAction {
  MoveTo(Box<dyn Location>),
  RedrawScreen(),
  ShowMessage(MessageType),
  RedrawWithMessage(MessageType),
  WinGame,
}
