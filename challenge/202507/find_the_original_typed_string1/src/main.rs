fn possible_string_count(word: String) -> i32 {
    let mut ret = 1;
    let mut count = 1;
    let mut prev = '?';

    for c in word.chars() {
        if c == prev {
            count += 1;
        } else {
            ret += count - 1;
            prev = c;
            count = 1;
        }
    }

    ret += count - 1;
    ret
}

fn main() {
    let ret = possible_string_count("abbcccc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(possible_string_count("abbcccc".to_string()), 5);
    assert_eq!(possible_string_count("abcd".to_string()), 1);
    assert_eq!(possible_string_count("aaaa".to_string()), 4);
}
