extern crate md5;
extern crate rustc_serialize;

use rustc_serialize::hex::ToHex;
use std::{iter, i32};
use std::io::{self, Read};

fn hash(key: &str, number: i32) -> md5::Digest {
    let data = format!("{}{}", key, number).into_bytes();

    md5::compute(&data)
}

#[test]
fn test_hash() {
    let examples: Vec<(&str, i32, &str)> = vec![
        ( "abcdef", 609043, "000001dbbfa" ),
        ( "pqrstuv", 1048970, "000006136ef" ),
    ];

    for (key, number, reference_result) in examples.into_iter() {
        let result = hash(key, number).to_hex();

        assert!(result.starts_with(reference_result));
    }
}

#[inline]
fn find_sufficient_number(key: &str, zeroes: usize) -> Result<i32, String> {
    let prefix: String = iter::repeat('0').take(zeroes).collect();
    let max: i32 = i32::MAX;

    for i in 0..max {
        if hash(key, i).to_hex().starts_with(&prefix) {
            return Ok(i);
        }
    }

    Err(format!("no sufficient number found in range: 0..{}", max))
}

#[test]
fn test() {
    let examples: Vec<(&str, usize, i32)> = vec![
        ( "abcdef", 5, 609043 ),
        ( "pqrstuv", 5, 1048970 ),
    ];

    for (key, zeroes, reference_number) in examples.into_iter() {
        let number = find_sufficient_number(key, zeroes);

        assert_eq!(number, Ok(reference_number));
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let key: String = buf.chars().filter(|c| c.is_alphabetic()).collect();

    println!("{}", find_sufficient_number(&key, 5).unwrap());
}
