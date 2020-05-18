mod action;
mod game;
mod item;
mod location;
mod locations;
mod state;

use game::{Game, confirm, init_terminal};

const INTRO: &str = r#"
==========================================================================================================
  _____     _                 _               _                 _                    ___   ___ ___   ___  
 |_   _|   | |               | |     /\      | |               | |                  |__ \ / _ \__ \ / _ \ 
   | |  ___| | __ _ _ __   __| |    /  \   __| |_   _____ _ __ | |_ _   _ _ __ ___     ) | | | | ) | | | |
   | | / __| |/ _` | '_ \ / _` |   / /\ \ / _` \ \ / / _ \ '_ \| __| | | | '__/ _ \   / /| | | |/ /| | | |
  _| |_\__ \ | (_| | | | | (_| |  / ____ \ (_| |\ V /  __/ | | | |_| |_| | | |  __/  / /_| |_| / /_| |_| |
 |_____|___/_|\__,_|_| |_|\__,_| /_/    \_\__,_| \_/ \___|_| |_|\__|\__,_|_|  \___| |____|\___/____|\___/ 
==========================================================================================================

You are an adventurer, set to explore the ancient island in the Atlantic Ocean.

Your mission: recover the lost Father's Day card.

The card is said to be located in the old fortress and protected by a powerful
dragon. So recovering the card will not be easy. You will have to explore
the island to discover the tools and weapons necessary to complete your task.

The island is displayed using ASCII (text) characters. Every object that you
can interact with is displayed in plain English. To navigate, you must type
commands into the terminal. You can perform the following actions:

  go   <place>         = Move to a new location
  take <item>          = Pick up an item
  use  <item>          = Use or interact with an item in the room
  talk <person>        = Speak to a person
  give <person> <item> = Give an item to a person
  inventory            = Print all items in inventory

If at any time you forget these commands, you can type "help" in the terminal.

"#;

const CARD: &str = r#"
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~ Hello Player,                                 ~
~ Thank you for trying my game! This is where   ~
~ Dad's card would normally go. However, I have ~
~ removed the card from the source tree since   ~
~ it is a special message only to my Dad. I     ~
~ hope you don't feel cheated and still had a   ~
~ special Father's Day yourself...              ~
~                     Sincerely,                ~
~                       Bryan                   ~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

fn main() {

  init_terminal();
  show_intro();
  if !confirm("Are you up to the challenge???") {
    println!("Okay! Bye Bye...");
    return;
  }

  let g = Game::new();
  if g.run() {
    show_card();
  }
}

fn show_intro() {
  print!("\x1B[2J\x1B[1;1H");
  println!("{}", INTRO);
}

fn show_card() {
  print!("\x1B[2J\x1B[1;1H");
  println!("{}", CARD);
}
