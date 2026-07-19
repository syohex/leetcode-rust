fn smallest_subsequence(s: String) -> String {
    let mut count = [0; 26];
    for b in s.bytes() {
        count[(b - b'a') as usize] += 1;
    }

    let mut checked = [false; 26];
    let mut ret = String::new();
    for c in s.chars() {
        let idx = c as usize - 'a' as usize;
        if !checked[idx] {
            while !ret.is_empty() {
                let last = ret.chars().last().unwrap();
                if last <= c {
                    break;
                }

                let idx2 = last as usize - 'a' as usize;
                if count[idx2] == 0 {
                    break;
                }

                checked[last as usize - 'a' as usize] = false;
                ret.pop();
            }

            checked[idx] = true;
            ret.push(c);
        }

        count[idx] -= 1;
    }
    ret
}

fn main() {
    let ret = smallest_subsequence("cbacdcbc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_subsequence("bcabc".to_string()), "abc");
    assert_eq!(smallest_subsequence("cbacdcbc".to_string()), "acdb");
}
