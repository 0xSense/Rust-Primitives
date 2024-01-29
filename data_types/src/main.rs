// data types
// rust applications 
fn main() {
  let guess: u32 = "42".parse().expect("Not a number");

  let stuff = (10, 3.14, 'x');

  say_hello();
}

fn say_hello() {
  println!("Hello!")
}
