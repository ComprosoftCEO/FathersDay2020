use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const beach3_image: &str = r#"

      ||           | |    ___  |     |  | |      
      ||           | |   /   \ |     |  | |      
      ||           | |   | !!| |_____|  | |      
______||           | |___\!~~!_/      \ | |      
      \|___________|/     !~~!         \|_|      
<-left           `        !~~!       .    \______
   `                .     !~~! .            `    
      .      `            !~~!        `   right->
     ~~       ~  .        !~~!  ~~~~   ~~~       
 .  ~~~~~ ~  ~~~~  ~~~~~~~~~~~~~~~~~~  ~~~~   ~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"#;

pub fn beach3() -> Box<dyn Location> {}
