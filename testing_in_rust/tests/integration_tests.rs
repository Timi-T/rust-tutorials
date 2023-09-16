use testing_in_rust;
mod common;

#[test]
fn integration_test_1() {
  common::setup();
  assert_eq!(testing_in_rust::add_two(3), 5);
}

// To run only integration tests we use --test crate_name
