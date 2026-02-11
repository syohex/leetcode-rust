fn reverse_by_type(s: String) -> String {
    let len = s.len();
    let cs: Vec<_> = s.chars().collect();
    let mut ret: Vec<char> = cs.clone();

    let mut left = 0;
    let mut right = len - 1;
    while left < right {
        while left < len && !cs[left].is_ascii_alphabetic() {
            left += 1;
        }
        while right > 0 && !cs[right].is_ascii_alphabetic() {
            right -= 1;
        }

        if left >= right {
            break;
        }

        ret[left] = cs[right];
        ret[right] = cs[left];
        left += 1;
        right -= 1;
    }

    left = 0;
    right = len - 1;
    while left < right {
        while left < len && cs[left].is_ascii_alphabetic() {
            left += 1;
        }
        while right > 0 && cs[right].is_ascii_alphabetic() {
            right -= 1;
        }

        if left >= right {
            break;
        }

        ret[left] = cs[right];
        ret[right] = cs[left];
        left += 1;
        right -= 1;
    }

    ret.into_iter().collect()
}

fn main() {
    let ret = reverse_by_type(")ebc#da@f(".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(reverse_by_type("(q".to_string()), "(q");
    assert_eq!(reverse_by_type(")ebc#da@f(".to_string()), "(fad@cb#e)");
    assert_eq!(reverse_by_type("z".to_string()), "z");
    assert_eq!(reverse_by_type("!@#$%^&*()".to_string()), ")(*&^%$#@!");
}
