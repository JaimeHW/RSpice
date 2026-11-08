use super::natural_cmp;
use std::cmp::Ordering;

fn sorted<'a>(names: &[&'a str]) -> Vec<&'a str> {
    let mut names = names.to_vec();
    names.sort_by(|left, right| natural_cmp(left, right));
    names
}

#[test]
fn digit_runs_count_as_numbers() {
    assert_eq!(
        sorted(&["nch.10", "nch.2", "nch.1"]),
        ["nch.1", "nch.2", "nch.10"]
    );
    assert_eq!(sorted(&["M10", "M2"]), ["M2", "M10"]);
}

#[test]
fn letters_compare_without_regard_to_case() {
    assert_eq!(natural_cmp("m1", "M2"), Ordering::Less);
    assert_eq!(natural_cmp("Rload", "rs"), Ordering::Less);
}

#[test]
fn leading_zeros_do_not_change_the_number() {
    assert_eq!(sorted(&["Q010", "Q9"]), ["Q9", "Q010"]);
}

#[test]
fn a_name_ends_before_its_extension() {
    assert_eq!(natural_cmp("M", "M1"), Ordering::Less);
    assert_eq!(natural_cmp("M1", "M"), Ordering::Greater);
}

#[test]
fn names_equal_to_the_reader_still_have_one_order() {
    assert_eq!(natural_cmp("M1", "m1"), "M1".cmp("m1"));
    assert_eq!(natural_cmp("Q01", "Q1"), "Q01".cmp("Q1"));
    assert_eq!(natural_cmp("M1", "M1"), Ordering::Equal);
}
