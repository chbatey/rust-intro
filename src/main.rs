extern crate intro;
use intro::add_one;

#[cfg(not(test))]
fn main() {

   let x = 6i;

   let y: int = if x == 6i {
      10i
   } else {
      11i
   };

   let z = add_one(x);

   println!("hey x {}", x);
   println!("hey y {}", y);
   println!("hey z {}", z);
}
