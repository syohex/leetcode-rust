fn max_score(s: String) -> i32 {
    let mut ones = s.chars().filter(|c| *c == '1').count();
    let len = s.len();

    let mut ret = 0;
    let mut zeros = 0;
    for c in s.chars().take(len - 1) {
        if c == '0' {
            zeros += 1;
        } else {
            ones -= 1;
        }

        ret = std::cmp::max(ret, zeros + ones);
    }

    ret as i32
}

fn main() {
    let ret = max_score("011101".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_score("011101".to_string()), 5);
    assert_eq!(max_score("00111".to_string()), 5);
    assert_eq!(max_score("1111".to_string()), 3);
}
