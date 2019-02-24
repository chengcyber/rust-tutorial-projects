fn main() {
  let mut x = 5;

  println!("The value of x is: {}", x);

  x = 6;

  println!("The value of x is: {}", x);


  // shadowing can change the type of the value
  let spaces = "     ";
  let spaces = spaces.len();

  println!("The lenght of spaces is: {}", spaces);
}
