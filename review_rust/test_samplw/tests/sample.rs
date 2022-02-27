use test_samplw;
mod common;

#[test]
fn it_adds_two() {
    let a = common::set_up();

    assert_eq!(4, a);
}