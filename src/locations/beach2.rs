use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const beach2_image: &str = &r#"
        .               ^                        
   starfish  `         up                        
     ^                              .     .      
    =*=        .              .                 `
    /^\              `                           
<-left                      .      .    ` right->
           .        .                            
  `                          ~          .        
 ~~~~~        ~~~      `   ~~~~~~   ~         ~~ 
~~~~~~~~~~  ~~~~~~~ ~~ ~~~~~~~~~~ ~~~~~      ~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach2() -> Box<dyn Location> {}
