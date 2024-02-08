use super::*;

#[test]
fn combine() {
    assert_eq!(769, two_byte_combine(3, 1));
}

#[test]
fn split() {
    assert_eq!((3, 1), two_byte_split(769));
}

#[test]
fn back_and_forth() {
    let aim = 30_000;

    let (split_1, split_2) = two_byte_split(aim);
    let combined = two_byte_combine(split_1, split_2);

    assert_eq!(aim, combined);
}