fn num_sub(s: String) -> i32 {
    let modulo = 1_000_000_007i64;
    let mut ret = 0i64;
    let mut ones = 0i64;

    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            ones += 1;
        }

        if (ones >= 1 && c == '0') || i == s.len() - 1 {
            let sum = ((ones * (1 + ones)) / 2) % modulo;
            ret = (ret + sum) % modulo;
            ones = 0;
        }
    }

    ret as i32
}

fn main() {
    let ret = num_sub("111110101011111111111010101".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_sub("0110111".to_string()), 9);
    assert_eq!(num_sub("101".to_string()), 2);
    assert_eq!(num_sub("111111".to_string()), 21);
}
