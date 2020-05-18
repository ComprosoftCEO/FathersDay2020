use crate::location::{Location, LocationBuilder};

const PLAINS1_IMAGE: &str = r#"
                                                 
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

pub fn plains1() -> Box<dyn Location> {
  LocationBuilder::new("Grassy Plains", PLAINS1_IMAGE)
    .add_location("down", super::beach1)
    .add_location("right", super::plains2)
    .finish()
}
