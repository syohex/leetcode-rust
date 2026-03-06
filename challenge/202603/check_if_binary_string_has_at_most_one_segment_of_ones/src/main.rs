fn check_one_segment(s: String) -> bool {
    let cs: Vec<_> = s.chars().collect();
    let mut i = 0;
    let mut checked = false;
    while i < cs.len() {
        if cs[i] == '1' {
            if checked {
                return false;
            }

            checked = true;
            i += 1;
            while i < cs.len() && cs[i] == '1' {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    checked
}

fn main() {
    let ret = check_one_segment("101100".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!check_one_segment("1001".to_string()));
    assert!(check_one_segment("110".to_string()));
    assert!(check_one_segment("1".to_string()));
    assert!(check_one_segment("10".to_string()));
    assert!(!check_one_segment("0000".to_string()));
}
