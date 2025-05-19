use crate::GreenhouseConfig;
use std::rc::Rc;

pub fn alert(config: &Rc<GreenhouseConfig>) {
    println!("Alert config {:?}", config);
    
}