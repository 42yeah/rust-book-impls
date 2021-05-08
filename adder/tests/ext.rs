mod commons;

use adder;
use commons::setup;

#[test]
fn it_adds_two() {
    setup();
    assert_eq!(4, adder::add_two(2));
}
