mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

// use bring absolute path
use crate::sound::instrument;
// use bring relative path, should be prefixed with self
use self::sound::instrument as relativeInstrument;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();

    instrument::clarinet();
    relativeInstrument::clarinet();

    //////////////
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
