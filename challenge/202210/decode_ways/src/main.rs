fn num_decodings(s: String) -> i32 {
    use std::collections::HashMap;

    fn f(pos: usize, cs: &Vec<char>, cache: &mut HashMap<usize, i32>) -> i32 {
        if pos >= cs.len() {
            return 1;
        }

        if let Some(v) = cache.get(&pos) {
            return *v;
        }

        let ret = match cs[pos] {
            '0' => 0,
            '1' => {
                if pos + 1 < cs.len() {
                    f(pos + 1, cs, cache) + f(pos + 2, cs, cache)
                } else {
                    f(pos + 1, cs, cache)
                }
            }
            '2' => {
                if pos + 1 < cs.len() && cs[pos + 1] >= '0' && cs[pos + 1] <= '6' {
                    f(pos + 1, cs, cache) + f(pos + 2, cs, cache)
                } else {
                    f(pos + 1, cs, cache)
                }
            }
            _ => f(pos + 1, cs, cache),
        };

        cache.insert(pos, ret);
        ret
    }

    let cs: Vec<char> = s.chars().collect();
    let mut cache = HashMap::new();
    f(0, &cs, &mut cache)
}

fn main() {
    let s = "12".to_string();
    let ret = num_decodings(s);
    println!("ret={ret}");
}

#[test]
fn test_num_decodings() {
    {
        let s = "12".to_string();
        let ret = num_decodings(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "226".to_string();
        let ret = num_decodings(s);
        assert_eq!(ret, 3);
    }
    {
        let s = "06".to_string();
        let ret = num_decodings(s);
        assert_eq!(ret, 0);
    }
    {
        let s = "1201234".to_string();
        let ret = num_decodings(s);
        assert_eq!(ret, 3);
    }
}
