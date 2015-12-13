use std::io::{self, BufRead};

fn wrapping_paper_for_box(box_dimensions: [i32; 3]) -> i32 {
    let w = box_dimensions[0];
    let l = box_dimensions[1];
    let h = box_dimensions[2];

    let wl = w * l;
    let wh = w * h;
    let lh = l * h;

    let extra = vec![wl, wh, lh].into_iter().min().unwrap();

    2 * (wl + wh + lh) + extra
}

#[test]
fn test_wrapping_paper_for_box() {
    let examples = [
        ([2, 3, 4], 58),
        ([1, 1, 10], 43),
    ];

    for &(b, w) in examples.iter() {
        assert_eq!( wrapping_paper_for_box(b), w );
    }
}

fn parse_box_dimensions(s: &str) -> Result<[i32; 3], String> {
    let result: Result<Vec<i32>, _> =
        s.split('x').map(str::parse).collect();

    match result {
        Ok(vec) => {
            if vec.len() != 3 {
                Err(String::from("wrong number of dimensions"))
            }
            else { Ok([vec[0], vec[1], vec[2]]) }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test_parse_box_dimensions() {
    let examples = [
        ("2x3x4", [2, 3, 4]),
        ("1x1x10", [1, 1, 10]),
    ];
    let bad_examples = [
        "hello!",
        "1x1",
        "1x2x3x4",
    ];

    for &(s, v) in examples.iter() {
        assert_eq!( parse_box_dimensions(s), Ok(v) );
    }

    for &s in bad_examples.iter() {
        assert!( parse_box_dimensions(s).is_err() );
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Result<Vec<_>, _> = stdin.lock().lines().collect();
    let box_dimensions: Result<Vec<_>, _> = lines.unwrap().iter()
        .map(|d| parse_box_dimensions(&d)).collect();

    match box_dimensions {
        Err(e) => { println!("{}", e); }
        Ok(box_dimensions) => {
            let total_wrapping_paper = box_dimensions.into_iter()
                .map(wrapping_paper_for_box)
                .fold(0, |acc, i| acc + i);
            println!("{}", total_wrapping_paper);
        }
    }
}
