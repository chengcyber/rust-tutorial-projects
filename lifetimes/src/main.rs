fn main() {
    lifetime_definition();
    lifetime_in_structs();
}

fn lifetime_definition() {

    let string1 = String::from("abcdefg");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 'a specify the lifetime among s1, s2, and return lifetime
    // lifetime syntax is about connecting the lifetimes of various parameters
    // and return values of function. Once they are connected, Rust has enough
    // information to allow memory-safe operations and disallow operations that
    // would create dangling pointers or otherwise violate memory safety.
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }
}


fn lifetime_in_structs() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    impl<'a> ImportantExcerpt<'a> {
        // elision rule 1 applied, no need specify lifetime
        fn level(&self) -> i32 {
            3
        }

        // elision rule 3 applied, no need specify lifetime
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
        // same as fn announce_and_return_part(&'a self, announcement: &'b str) -> &'a str {}
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
                              .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

fn static_lifetime() {
    // stirng literals is always available
    let s: &'static str = "I have a static lifetime";
}
