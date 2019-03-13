// tests directory at the top level of the project directory, next to src/,
// Cargo knows to look for integration test files in this directory.

// cargo test --test integration_test to run this file only

// import the src project as external crate, it names TESTS
extern crate tests;
// import the path of module adder in src/
use tests::adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
