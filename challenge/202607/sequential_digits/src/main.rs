fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let digits = "123456789";
    let (low_s, high_s) = (low.to_string(), high.to_string());
    let (low_digits, high_digits) = (low_s.len(), high_s.len());

    let mut ret = vec![];
    for i in low_digits..=high_digits {
        for j in 0..(digits.len() - i + 1) {
            let s = &digits[j..(j + i)];
            if let Ok(v) = s.parse()
                && v >= low
                && v <= high
            {
                ret.push(v);
            }
        }
    }

    ret
}

fn main() {
    let ret = sequential_digits(1000, 13000);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(sequential_digits(100, 300), [123, 234]);
    assert_eq!(
        sequential_digits(1000, 13000),
        [1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
