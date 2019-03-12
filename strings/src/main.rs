fn main() {
    creating();
    appending();
    iterating();
}

fn creating() {
    let mut _s = String::new();
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    // remember that strings are UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}

fn appending() {
    let mut s = String::from("foo");
    let s2 = "bar";
    // push_str takes string slice of s2, instead of ownership
    s.push_str(s2);
    println!("s is {}", s);
    println!("s2 is {}", s2);

    // push a single character
    let mut lol = String::from("lo");
    lol.push('l');
    println!("lol ... {}", lol);


    // concatenation with + operator
    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let str3 = str1 + &str2; // note str1 has been moved here and can no longer be used
    println!("{}", str3);

    // format! macro returns a String with the contents,
    // while not take ownership of any of its parameters.
    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");
    let strf = format!("{}-{}-{}", ss1, ss2, ss3);
    println!("{}", strf);
}

fn iterating() {
    // NOTE, indexing into String could not compile in Rust

    println!("---- chars -----");
    // chars returns six values of type char
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("---- bytes -----");
    // bytes returns ecah raw byte
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
