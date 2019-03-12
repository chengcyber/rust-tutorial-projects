use std::io::{self, Read, ErrorKind};
use std::fs::{self, File};
use std::error::Error;

// The ? operator can only be used in functions that have a return type of Result
// Box<dyn Error> means "any kind of error."
fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;

    Ok(())
}


fn nest_matches() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}

fn nest_matches_shortcuts() {
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok,
    // If the Result is Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();

    // We use expect in the same way s unwrap, but expect can provide good error messages
    let f = File::open("hello.txt").expect("failed to open hello.txt");
}

// The return is either an Ok value that contains a username or
// an Err value that contains an io::Error
// we propagate all the success or error information upward for it to handle appropriately.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e)  => Err(e),
    }
}

// the ? Operator, a shortcut for propagating erros
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// chain method calls after ?
fn read_username_from_file_shortshortcut() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Reading a file into a string is a farily common operation,
// so Rust provide a convenience function called fs::read_to_string
// that will open the file, create a new String, read the contents
// of the file, and put the contents into that String, and then return it.
fn read_username_from_file_shortshortshortcut() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
