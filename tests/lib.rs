extern crate intro;
use intro::add_one;

#[test]
fn one_plus_one_is_two() {
    let result = add_one(1);
    assert_eq!(2, result);
}