fn min_operations(s: String) -> i32 {
    let mut c0 = 0;
    let mut c1 = 0;
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            if c == '1' {
                c0 += 1;
            } else {
                c1 += 1;
            }
        } else {
            if c == '0' {
                c0 += 1;
            } else {
                c1 += 1;
            }
        }
    }

    std::cmp::min(c0, c1)
}

fn main() {
    println!(
        "min_operations('0100')={}",
        min_operations("0100".to_string())
    );
}

#[test]
fn test_min_operations() {
    assert_eq!(min_operations("0100".to_string()), 1);
    assert_eq!(min_operations("10".to_string()), 0);
    assert_eq!(min_operations("1111".to_string()), 2);
}
