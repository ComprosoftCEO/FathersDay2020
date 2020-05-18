use rl_sys::history;
use rl_sys::readline::readline;

use crate::location::{GameAction, Location};
use crate::state::State;

extern "C" {
  pub static mut rl_inhibit_completion: libc::c_int;
}

pub struct Game {
  location: Box<dyn Location>,
  state: State,
  last_command: String,
  running: bool,
  winner: bool,
}

impl Game {
  pub fn new() -> Self {
    Game {
      location: crate::locations::plains2(),
      state: State::new(),
      last_command: "".into(),
      running: true,
      winner: false,
    }
  }

  pub fn run(mut self) -> bool {
    Game::init_terminal();

    self.print_current_location();
    while self.running {
      self.read_and_execute_command();
    }

    self.winner
  }

  fn init_terminal() {
    unsafe {
      rl_inhibit_completion = 1;
    }
  }

  fn read_and_execute_command(&mut self) {
    match self.read_line() {
      None => return,
      Some(line) => self.execute_command(line),
    }
  }

  fn read_line(&mut self) -> Option<String> {
    let readline_data = readline("> ");
    let line: String = match readline_data {
      Err(e) => {
        println!("{}", e);
        self.running = false;
        return None;
      }
      Ok(data) => match data {
        Some(s) => s,
        None => "".into(),
      },
    };

    // Remove excess whitespace
    let trimmed_line = line.trim();
    if trimmed_line.len() == 0 {
      return None;
    }

    // Add the line to history
    if trimmed_line != self.last_command {
      history::listmgmt::add(&line).expect("Error adding to history!");
      self.last_command = trimmed_line.into();
    }

    Some(trimmed_line.into())
  }

  fn execute_command(&mut self, command: String) {
    let arguments: Vec<&str> = command.split_whitespace().collect();
    match arguments[0] {
      "help" => {
        if test_arguments("edit", arguments.len(), 0) {
          show_help();
        }
      }
      "exit" => {
        if test_arguments("edit", arguments.len(), 0) && confirm("Really Exit?") {
          self.running = false;
        }
      }
      "inventory" => {
        if test_arguments("inventory", arguments.len(), 0) {
          println!("\nInventory:");
          self.state.list_inventory();
          print!("\n");
        }
      }
      "whereami" => {
        if test_arguments("whereami", arguments.len(), 0) {
          self.print_current_location()
        }
      }
      "go" => {
        if test_arguments("go", arguments.len(), 1) {
          let result = self.location.move_to(&mut self.state, arguments[1]);
          self.perform_action(result);
        }
      }
      "take" => {
        if test_arguments("take", arguments.len(), 1) {
          let result = self.location.pickup_item(&mut self.state, arguments[1]);
          self.perform_action(result);
        }
      }
      "use" => {
        if test_arguments("use", arguments.len(), 1) {
          let result = self.location.use_item(&mut self.state, arguments[1]);
          self.perform_action(result);
        }
      }
      "talk" => {
        if test_arguments("talk", arguments.len(), 1) {
          let result = self.location.talk_to(&mut self.state, arguments[1]);
          self.perform_action(result);
        }
      }
      "give" => {
        if test_arguments("give", arguments.len(), 2) {
          let result = self.location.give_to(&mut self.state, arguments[1], arguments[2]);
          self.perform_action(result);
        }
      }
      unknown => println!("Unknown command '{}'", unknown),
    }
  }

  fn perform_action(&mut self, action: GameAction) {
    match action {
      GameAction::MoveTo(l) => {
        self.location = l;
        self.print_current_location();
      }
      GameAction::RedrawScreen() => self.print_current_location(),
      GameAction::ShowMessage(msg) => println!("{}", msg.to_string()),
      GameAction::RedrawWithMessage(msg) => {
        self.print_current_location();
        println!("{}\n", msg.to_string());
      }
      GameAction::WinGame => {
        self.running = false;
        self.winner = true;
      }
    }
  }

  fn print_current_location(&self) {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", self.location.get_image(&self.state));
  }
}

fn show_help() {
  println!("\nAll Commands:");
  println!("  inventory            = Print all items in inventory");
  println!("  whereami             = Redraw the current location\n");
  println!("  go   <place>         = Move to a new location");
  println!("  take <item>          = Pick up an item");
  println!("  use  <item>          = Use or interact with an item in the room");
  println!("  talk <person>        = Speak to a person");
  println!("  give <person> <item> = Give an item to a person\n");
  println!("  help = Help screen");
  println!("  exit = End the game\n");
}

fn confirm(prompt: &'static str) -> bool {
  let prompt: String = format!("{} [Y|N] ", prompt);
  loop {
    let data = readline(&*prompt);
    let line: String = match data {
      Err(_) => continue,
      Ok(l) => match l {
        Some(s) => s,
        None => "".into(),
      },
    };

    let trimmed = line.trim();
    if trimmed.len() == 0 {
      continue;
    }

    if (trimmed == "y") || (trimmed == "Y") {
      return true;
    }
    if (trimmed == "n") || (trimmed == "N") {
      return false;
    }
  }
}

fn test_arguments(command: &'static str, num: usize, num_expected: usize) -> bool {
  if (num - 1) != num_expected {
    println!("{}: expected {} arguments, {} given", command, num_expected, num - 1);
    return false;
  }
  return true;
}
