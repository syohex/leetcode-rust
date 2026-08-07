fn count_valid_prefixes(s: String) -> i32 {
    s.chars()
        .fold((0, 0i32, 0), |(acc, zeros, ones), c| {
            let (zeros, ones) = if c == '0' {
                (zeros + 1, ones)
            } else {
                (zeros, ones + 1)
            };

            let m = (zeros + ones) % 2;
            if (zeros - ones).abs() == m {
                (acc + 1, zeros, ones)
            } else {
                (acc, zeros, ones)
            }
        })
        .0
}

fn main() {
    let ret = count_valid_prefixes("00101".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_valid_prefixes("00101".to_string()), 3);
    assert_eq!(count_valid_prefixes("101".to_string()), 3);
}
