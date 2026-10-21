fn min_operations(s: String) -> i32 {
    let mut cs: Vec<_> = s.bytes().collect();
    let mut min_char = cs[0];
    let mut max_char = cs[0];
    let mut freq = [0; 26];
    freq[(cs[0] - b'a') as usize] += 1;
    for &c in cs.iter().skip(1) {
        min_char = std::cmp::min(min_char, c);
        max_char = std::cmp::max(max_char, c);
        freq[(c - b'a') as usize] += 1;
    }

    let orig = cs.clone();
    cs.sort_unstable();
    let min_chars = freq[(min_char - b'a') as usize];
    let max_chars = freq[(max_char - b'a') as usize];

    let len = cs.len();
    if orig == cs {
        0
    } else if orig[0] == min_char || orig[len - 1] == max_char {
        1
    } else if len == 2 {
        -1
    } else if orig[len - 1] == min_char && orig[0] == max_char && min_chars == 1 && max_chars == 1 {
        3
    } else {
        2
    }
}

fn main() {
    let ret = min_operations("dog".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_operations("edc".to_string()), 3);
    assert_eq!(min_operations("jgg".to_string()), 2);
    assert_eq!(min_operations("dog".to_string()), 1);
    assert_eq!(min_operations("card".to_string()), 2);
    assert_eq!(min_operations("gf".to_string()), -1);
}
