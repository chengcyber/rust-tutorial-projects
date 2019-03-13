
pub mod adder {
    pub fn add_two(x: i32) -> i32 {
        x + 2
    }
}

// Unit Tests
// This annotation tells Rust to compile and run the code
// only when you run cargo test
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unit_works() {
        assert_eq!(adder::add_two(2), 4);
    }
}
