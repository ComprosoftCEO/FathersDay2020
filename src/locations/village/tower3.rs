use crate::action::Action;
use crate::item::Item;
use crate::location::{Location, LocationBuilder};
use crate::state::State;

const tower3_image: &str = r#"
            _________________________            
           /                         \           
          /                           \          
         /  |  |                       \         
        /   |==| ladder                 \        
       /    |  |                         \       
      |     |==|                          |      
      |     |  |                          |      
      |     |==|                          |      
      |_____|__|_______down_______________|      
     /                  V      ____|       \     
"#;

pub fn tower3() -> Box<dyn Location> {}
