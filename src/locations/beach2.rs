use crate::location::{Location, LocationBuilder};
use crate::locations::helpers::collect_starfish;
use crate::state::State;

const BEACH2_IMAGE: &str = r#"
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

const BEACH2_NO_STARFISH: &str = r#"
        .               ^                        
     `       `         up                        
                                    .     .      
  `     `      .              .                 `
                     `                           
<-left                      .      .    ` right->
           .        .                            
  `                          ~          .        
 ~~~~~        ~~~      `   ~~~~~~   ~         ~~ 
~~~~~~~~~~  ~~~~~~~ ~~ ~~~~~~~~~~ ~~~~~      ~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach2() -> Box<dyn Location> {
  LocationBuilder::new_dynamic("Beach", get_image)
    .add_location("up", crate::locations::plains2)
    .add_location("left", crate::locations::beach1)
    .add_location("right", crate::locations::beach3)
    .add_dynamic_item("starfish", collect_starfish(1))
    .finish()
}

pub fn get_image(state: &State) -> String {
  if state.has_collected_starfish(1) {
    BEACH2_NO_STARFISH.into()
  } else {
    BEACH2_IMAGE.into()
  }
}
