fn max_sum_of_squares(num: i32, sum: i32) -> String {
    let mut sum = sum;
    let mut ret = String::new();

    for _ in 0..num {
        let v = std::cmp::min(9, sum);
        let c = char::from_digit(v as u32, 10).unwrap();
        ret.push(c);
        sum -= v;
    }

    if sum == 0 { ret } else { String::new() }
}

fn main() {
    let ret = max_sum_of_squares(2, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_sum_of_squares(2, 3), "30");
    assert_eq!(max_sum_of_squares(2, 17), "98");
    assert_eq!(max_sum_of_squares(1, 10), "");
}
