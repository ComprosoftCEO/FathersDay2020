use crate::location::{Location, LocationBuilder};

const FOREST_IMAGE: &str = r#"
   ()    ()        )    ^   (    ()        ()    
       ()    ))))))    up    ((     ()       ()  
  ()()   ))))              *   (((      ()  ()() 
______)))         *         \/    ((((___________
              \/                                 
<-left                  `           `     right->
________    *                             _______
  ()    )        \/        *  \/  ((((((((    () 
         )))                 (((((        ()     
    ()      ))))      down  (   ()   ()       () 
()    ()()      )       V  (  ()  ()        ()   
"#;

enum Direction {
  Up,
  Down,
  Left,
}

pub fn forest_maze() -> Box<dyn Location> {
  build_direction(Direction::Left, maze1)
}

fn build_direction(dir: Direction, next_path: impl Fn() -> Box<dyn Location> + 'static) -> Box<dyn Location> {
  let mut builder = LocationBuilder::new("Dense Forest", FOREST_IMAGE);

  match dir {
    Direction::Up => {
      builder = builder.add_location("up", next_path);
    }
    Direction::Down => {
      builder = builder.add_location("down", next_path);
    }
    Direction::Left => {
      builder = builder.add_location("left", next_path);
    }
  }

  builder
    .add_location_no_duplicate("up", forest_maze)
    .add_location_no_duplicate("down", forest_maze)
    .add_location_no_duplicate("left", forest_maze)
    .add_location_no_duplicate("right", super::village_outside)
    .finish()
}

//
// Store maze state as a simple finite state machine
//
fn maze1() -> Box<dyn Location> {
  build_direction(Direction::Up, maze2)
}

fn maze2() -> Box<dyn Location> {
  build_direction(Direction::Left, maze3)
}

fn maze3() -> Box<dyn Location> {
  build_direction(Direction::Down, maze4)
}

fn maze4() -> Box<dyn Location> {
  build_direction(Direction::Left, maze5)
}

fn maze5() -> Box<dyn Location> {
  build_direction(Direction::Left, super::temple_ruins)
}
