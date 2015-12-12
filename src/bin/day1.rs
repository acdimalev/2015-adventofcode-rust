use std::io::{self, Read};

fn char_value(c: char) -> i64 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

fn str_value(s: &str) -> i64 {
    s.chars().map(char_value).fold(0, |acc, v| acc + v)
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", str_value(&s));
}

#[test]
fn test() {
    let str_values = [
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ];
    for &(s, v) in str_values.iter() {
        assert_eq!( str_value(&s), v );
    }
}
