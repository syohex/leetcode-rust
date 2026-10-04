fn check_valid_string(s: String) -> bool {
    use std::collections::HashMap;

    fn f(pos: usize, opens: usize, s: &[char], cache: &mut HashMap<(usize, usize), bool>) -> bool {
        if pos >= s.len() {
            return opens == 0;
        }

        let key = (pos, opens);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let ret = match s[pos] {
            '*' => {
                if f(pos + 1, opens + 1, s, cache) || f(pos + 1, opens, s, cache) {
                    true
                } else if opens >= 1 {
                    f(pos + 1, opens - 1, s, cache)
                } else {
                    false
                }
            }
            '(' => f(pos + 1, opens + 1, s, cache),
            _ => {
                if opens >= 1 {
                    f(pos + 1, opens - 1, s, cache)
                } else {
                    false
                }
            }
        };
        cache.insert(key, ret);
        ret
    }

    let mut cache = HashMap::new();
    let cs: Vec<_> = s.chars().collect();
    f(0, 0, &cs, &mut cache)
}

fn main() {
    let ret = check_valid_string("(((((****)))))".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_valid_string("()".to_string()));
    assert!(check_valid_string("(*)".to_string()));
    assert!(check_valid_string("(*))".to_string()));
    assert!(!check_valid_string("(".to_string()));
}
