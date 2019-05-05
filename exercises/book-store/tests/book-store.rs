//! Tests for book-store
//!
//! Generated by [script][script] using [canonical data][canonical-data]
//!
//! [script]: https://github.com/exercism/rust/blob/master/bin/init_exercise.py
//! [canonical-data]: https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/book-store/canonical_data.json

use book_store::*;

/// Process a single test case for the property `total`
///
/// All cases for the `total` property are implemented
/// in terms of this function.
///
/// Expected input format: ('basket', 'targetgrouping')
fn process_total_case(input: (Vec<u32>, Vec<Vec<u32>>), expected: u32) {
    assert_eq!(expected, lowest_price(&input.0))
}

// Return the total basket price after applying the best discount.
// Calculate lowest price for a shopping basket containing books only from
// a single series.  There is no discount advantage for having more than
// one copy of any single book in a grouping.

#[test]
/// Only a single book
fn test_only_a_single_book() {
    process_total_case((vec![1], vec![vec![1]]), 800);
}

#[test]
#[ignore]
/// Two of the same book
fn test_two_of_the_same_book() {
    process_total_case((vec![2, 2], vec![vec![2], vec![2]]), 1_600);
}

#[test]
#[ignore]
/// Empty basket
fn test_empty_basket() {
    process_total_case((vec![], vec![]), 0);
}

#[test]
#[ignore]
/// Two different books
fn test_two_different_books() {
    process_total_case((vec![1, 2], vec![vec![1, 2]]), 1_520);
}

#[test]
#[ignore]
/// Three different books
fn test_three_different_books() {
    process_total_case((vec![1, 2, 3], vec![vec![1, 2, 3]]), 2_160);
}

#[test]
#[ignore]
/// Four different books
fn test_four_different_books() {
    process_total_case((vec![1, 2, 3, 4], vec![vec![1, 2, 3, 4]]), 2_560);
}

#[test]
#[ignore]
/// Five different books
fn test_five_different_books() {
    process_total_case((vec![1, 2, 3, 4, 5], vec![vec![1, 2, 3, 4, 5]]), 3_000);
}

#[test]
#[ignore]
/// Two groups of four is cheaper than group of five plus group of three
fn test_two_groups_of_four_is_cheaper_than_group_of_five_plus_group_of_three() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 5],
            vec![vec![1, 2, 3, 4], vec![1, 2, 3, 5]],
        ),
        5_120,
    );
}

#[test]
#[ignore]
/// Group of four plus group of two is cheaper than two groups of three
fn test_group_of_four_plus_group_of_two_is_cheaper_than_two_groups_of_three() {
    process_total_case(
        (vec![1, 1, 2, 2, 3, 4], vec![vec![1, 2, 3, 4], vec![1, 2]]),
        4_080,
    );
}

#[test]
#[ignore]
/// Two each of first 4 books and 1 copy each of rest
fn test_two_each_of_first_4_books_and_1_copy_each_of_rest() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5],
            vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4]],
        ),
        5_560,
    );
}

#[test]
#[ignore]
/// Two copies of each book
fn test_two_copies_of_each_book() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
            vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]],
        ),
        6_000,
    );
}

#[test]
#[ignore]
/// Three copies of first book and 2 each of remaining
fn test_three_copies_of_first_book_and_2_each_of_remaining() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1],
            vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], vec![1]],
        ),
        6_800,
    );
}

#[test]
#[ignore]
/// Three each of first 2 books and 2 each of remaining books
fn test_three_each_of_first_2_books_and_2_each_of_remaining_books() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1, 2],
            vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], vec![1, 2]],
        ),
        7_520,
    );
}

#[test]
#[ignore]
/// Four groups of four are cheaper than two groups each of five and three
fn test_four_groups_of_four_are_cheaper_than_two_groups_each_of_five_and_three() {
    process_total_case(
        (
            vec![1, 1, 2, 2, 3, 3, 4, 5, 1, 1, 2, 2, 3, 3, 4, 5],
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 5],
            ],
        ),
        10_240,
    );
}
