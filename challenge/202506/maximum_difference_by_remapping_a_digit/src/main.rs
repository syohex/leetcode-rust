fn min_max_difference(num: i32) -> i32 {
    let s = num.to_string();
    let bs: Vec<_> = s.bytes().collect();

    let first_not_nine = bs.iter().find(|&&b| b != b'9').unwrap_or(&b'9');
    let max = bs.iter().fold(0, |acc, b| {
        if *b == *first_not_nine {
            acc * 10 + 9
        } else {
            acc * 10 + (b - b'0') as i32
        }
    });
    let min = bs.iter().fold(0, |acc, b| {
        if *b == bs[0] {
            acc * 10
        } else {
            acc * 10 + (b - b'0') as i32
        }
    });

    max - min
}

fn main() {
    let ret = min_max_difference(11891);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_max_difference(11891), 99009);
    assert_eq!(min_max_difference(90), 99);
}
