fn has_match(s: String, p: String) -> bool {
    fn f(pos1: usize, pos2: usize, cs: &[char], pattern: &[char]) -> bool {
        if pos2 >= pattern.len() {
            return true;
        }
        if pos1 >= cs.len() {
            return false;
        }

        if pattern[pos2] == '*' {
            for i in pos1..cs.len() {
                if f(i, pos2 + 1, cs, pattern) {
                    return true;
                }
            }
            false
        } else if cs[pos1] == pattern[pos2] {
            f(pos1 + 1, pos2 + 1, cs, pattern)
        } else {
            false
        }
    }

    let cs: Vec<_> = s.chars().collect();
    let mut pattern: Vec<_> = p.chars().skip_while(|&c| c == '*').collect();
    while !pattern.is_empty() {
        let c = *pattern.last().unwrap();
        if c == '*' {
            pattern.pop();
        } else {
            break;
        }
    }

    if pattern.is_empty() {
        return true;
    }

    for i in 0..cs.len() {
        if cs[i] == pattern[0] && f(i, 0, &cs, &pattern) {
            return true;
        }
    }

    false
}

fn main() {
    let ret = has_match("leetcode".to_string(), "ee*e".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(has_match("l".to_string(), "*".to_string()));
    assert!(has_match("xks".to_string(), "s*".to_string()));
    assert!(!has_match("nrnrs".to_string(), "*nn".to_string()));
    assert!(has_match("leetcode".to_string(), "ee*e".to_string()));
    assert!(!has_match("car".to_string(), "c*v".to_string()));
    assert!(has_match("luck".to_string(), "u*".to_string()));
}
