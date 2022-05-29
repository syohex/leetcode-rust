fn digit_count(num: String) -> bool {
    let counts = num.bytes().fold(vec![0; 10], |mut acc, b| {
        let index = (b - b'0') as usize;
        acc[index] += 1;
        acc
    });

    num.bytes()
        .enumerate()
        .all(|(i, b)| counts[i] == (b - b'0') as i32)
}

fn main() {
    let ret = digit_count("1210".to_string());
    println!("ret={ret}");
}

#[test]
fn test_digit_count() {
    assert!(digit_count("1210".to_string()));
    assert!(!digit_count("030".to_string()));
}
