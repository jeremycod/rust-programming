mod greetings;
mod utils;

fn main() {

   greetings::hello();
   greetings::hola();
   greetings::informal::saludo();
   greetings::formal::greet();
   
   use utils::math::*;
   let x1 = add(2, 4);
   println!("result is {:?}", x1);
  
   
   use utils::string_ops;
   
   let y1 = string_ops::reverse("to reverse this");
   println!("Reversed: {:?}", y1);
   
   
}
