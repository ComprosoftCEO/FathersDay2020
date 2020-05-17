use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const plains1_image: &str = r#"
                                                 
      ____         \/             \/             
     (    )                                      
      ( ()       `    `-----`-`-`-`----`-`--`-`--
  \/   ()           _ /   .              .       
      \||     \/  /                .      right->
       ||/        |                          .   
\/     ||         `   .    / ```-`--``----`--`---
                  |        |                     
                  `  down  |                     
           \/     |    V   |            \/       
"#;

pub fn plains1() -> Box<dyn Location> {}
