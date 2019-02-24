fn main() {

  basic_branch();

  return_branch();

  return_loop();

  while_loop();

  for_loop();

  countdown_for();

}

fn basic_branch() {

  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

}

fn return_branch() {

  let condition = true;

  // expression-orientied
  let num = if condition {
    5
  } else {
    6
  };

  println!("The value of num is: {}", num);

}

fn return_loop() {

  // loop

  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  assert_eq!(result, 20);
}

fn while_loop() {

  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number = number - 1;
  }

  println!("LIFTOFF!!!");
}

fn for_loop() {

  let a = [10, 20, 30, 40, 50];


  for element in a.iter() {
    println!("the value in a is: {}", element);
  }

}

fn countdown_for() {
  for number in (1..4).rev() {
    println!("countdown: {}!", number);
  }

  println!("LIFTOFF!!!");
}
