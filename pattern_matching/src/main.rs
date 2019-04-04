fn main() {
    match_literals_demo();
    match_named_var_demo();
    match_multiple_demo();
    match_range_demo();
    if_let_demo();
    while_let_demo();
    function_parameters_demo();
    destruct_struct_demo();
    destruct_enum_demo();
    destruct_nest_struct_and_enum();
    destruct_references_demo();
    destruct_struct_and_tuple_demo();
    extra_conditional_match_guard_demo();
    at_binding_demo();
}

fn match_literals_demo() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_named_var_demo() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // inner y binding to any value of x, rather than outter y
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn match_multiple_demo() {
    let x = 1;

    match x {
        // | means or
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_range_demo() {
    let x = 5;

    match x {
        // ... allows us to match an inclusive range of values
        // same as 1 | 2 | 3 | 4 | 5
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // ... only allow char and number values
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn if_let_demo() {

    main();

    fn main() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            // shadowed age should be use inside the block
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
}

fn while_let_demo() {

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn function_parameters_demo() {
    main();

    fn main() {
        let point = (3, 5);
        print_coordinates(&point);
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
}

fn destruct_struct_demo() {
    struct Point {
        x: i32,
        y: i32,
    }

    main();

    fn main() {
        let p = Point {
            x: 0,
            y: 7,
        };

        let Point { x: a, y: b } = p;

        assert_eq!(0, a);
        assert_eq!(7, b);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}

fn destruct_enum_demo() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    main();

    fn main() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure");
            },
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                );
            },
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, blue {}",
                    r,
                    g,
                    b,
                );
            }
        }
    }
}

fn destruct_nest_struct_and_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    main();

    fn main() {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b,
                );
            },
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change th color to hue {}, saturation {}, and value {}",
                    h,
                    s,
                    v,
                );
            },
            _ => ()
        }
    }
}

fn destruct_references_demo() {
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    println!("The sum of squares is: {}", sum_of_squares);
}

fn destruct_struct_and_tuple_demo() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
}

fn extra_conditional_match_guard_demo() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => ()
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_binding_demo() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            // id is not binding here
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
