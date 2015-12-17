use std::io::{self, BufRead};

fn string_contains_three_vowels(string: &str) -> bool {
    let vowels = "aeiou";
    let count = string.chars()
        .filter(|&c| vowels.contains(c))
        .count();

    count >= 3
}

#[test]
fn test_string_contains_three_vowels() {
    let examples: Vec<(&str, bool)> = vec![
        ("ugknbfddgicrmopn", true),
        ("aaa", true),
        ("dvszwmarrgswjxmb", false),
    ];

    for (string, reference_result) in examples.into_iter() {
        let result = string_contains_three_vowels(string);

        assert_eq!(result, reference_result);
    }
}

fn string_contains_double_letter(string: &str) -> bool {
    let iter1 = string.chars();
    let mut iter2 = iter1.clone();

    iter2.next();

    iter1
    .map(|c| Some(c) == iter2.next())
    .fold(false, |acc, x| acc || x)
}

#[test]
fn test_string_contains_double_letter() {
    let examples: Vec<(&str, bool)> = vec![
        ("ugknbfddgicrmopn", true),
        ("aaa", true),
        ("jchzalrnumimnmhp", false),
    ];

    for (string, reference_result) in examples.into_iter() {
        let result = string_contains_double_letter(string);

        assert_eq!(result, reference_result);
    }
}

fn string_contains_blacklisted_substring(string: &str) -> bool {
    let blacklisted_substrings = ["ab", "cd", "pq", "xy"];

    blacklisted_substrings
    .into_iter()
    .map(|s| string.contains(s))
    .fold(false, |acc, x| acc || x)
}

#[test]
fn test_string_contains_blacklisted_substring() {
    let examples: Vec<(&str, bool)> = vec![
        ("ugknbfddgicrmopn", false),
        ("aaa", false),
        ("haegwjzuvuyypxyu", true),
    ];

    for (string, reference_result) in examples.into_iter() {
        let result = string_contains_blacklisted_substring(string);

        assert_eq!(result, reference_result);
    }
}

fn string_is_nice(string: &str) -> bool {
    string_contains_three_vowels(string)
    && string_contains_double_letter(string)
    && ! string_contains_blacklisted_substring(string)
}

#[test]
fn test_string_is_nice() {
    let examples: Vec<(&str, bool)> = vec![
        ("ugknbfddgicrmopn", true),
        ("aaa", true),
        ("jchzalrnumimnmhp", false),
        ("haegwjzuvuyypxyu", false),
        ("dvszwmarrgswjxmb", false),
    ];

    for (string, should_be_nice) in examples.into_iter() {
        let is_nice = string_is_nice(string);

        assert_eq!(is_nice, should_be_nice);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Result<Vec<_>, _> = stdin.lock().lines().collect();

    let count = lines.unwrap().into_iter()
        .filter(|s| string_is_nice(s)).count();

    println!("{}", count);
}

#[test]
fn test() {
    let examples = [
        "ugknbfddgicrmopn",
        "aaa",
        "jchzalrnumimnmhp",
        "haegwjzuvuyypxyu",
        "dvszwmarrgswjxmb",
    ];
    let reference_result = 2;

    let result = examples.into_iter()
        .filter(|s| string_is_nice(s)).count();

    assert_eq!(result, reference_result);
}
