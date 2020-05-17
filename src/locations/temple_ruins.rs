use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const temple_ruins_image: &str = r#"
     /    _      _     \                   (   ()
    /    | |    | |     \       \/        (()    
   /     |%|    |$|      \               (    () 
  /  _   |_|    |_|   _   \  \/          (______(
 |  | |              | |  |         \/           
 |  |&|      ()      |#|  |               right->
 |  |_|   _      _   |_|  |               _______
 \       | |    | |      /             (((       
  \      |!|    |@|     /             (    ()    
   \     |_|    |_|    /       \/    (        () 
    \                 /  \/         (    ()      
"#;

pub fn temple_ruins() -> Box<dyn Location> {}
