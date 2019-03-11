fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // _ placeholder
    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("not three")
    }

}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
