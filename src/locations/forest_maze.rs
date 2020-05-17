use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const forest_image: &str = r#"
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

pub fn forest_maze() -> Box<dyn Location> {}
