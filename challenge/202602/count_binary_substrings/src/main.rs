fn count_binary_substrings(s: String) -> i32 {
    let cs : Vec<_> = s.chars().collect();
    let len = cs.len();

    let mut current = 1;
    let mut prev = 0;
    let mut ret = 0;
    for i in 1..len {
        if cs[i] != cs[i - 1] {
            ret += std::cmp::min(prev, current);
            prev = current;
            current = 1;
        } else {
            current += 1;
        }
    }

    ret + std::cmp::min(prev, current)
}

fn main() {
    let ret = count_binary_substrings("00110011".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_binary_substrings("00110".to_string()), 3);
    assert_eq!(count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(count_binary_substrings("10101".to_string()), 4);
}
