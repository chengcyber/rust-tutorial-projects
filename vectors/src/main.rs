fn main() {
    creating();
    reading();
    iterating();
    enum_multiple_types();
}

// how to create vectors
fn creating() {
    // generic version of creating an empty vector
    let _v: Vec<i32> = Vec::new();

    // infer Vec<i32> type when creating an vector
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.pop();

    println!("The vector: {:?}", v);
}

// reading elements of vectors
fn reading() {
    let v = vec![1, 2, 3, 4, 5];

    // gives us a reference of the element by index 2
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get method gives an Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}

// iterating over values in vector
fn iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", *i + 50);
    }
}

// Using enums to store multiple types
fn enum_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("Row vector is {:?}", row);
}
