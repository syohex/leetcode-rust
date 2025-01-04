fn count_palindromic_subsequences(s: String) -> i32 {
    let limit = s.len();
    let mut pos = [(limit, limit); 26];
    let bs: Vec<u8> = s.bytes().collect();
    for (i, &b) in bs.iter().enumerate() {
        let index = (b - b'a') as usize;
        if pos[index].0 == limit {
            pos[index].0 = i;
        } else {
            pos[index].1 = i;
        }
    }

    let mut ret = 0;
    for (s, e) in pos {
        if s == limit || e == limit {
            continue;
        }

        let mut checked = [false; 26];
        for &b in bs.iter().take(e).skip(s + 1) {
            let index = (b - b'a') as usize;
            if !checked[index] {
                ret += 1;
                checked[index] = true;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_palindromic_subsequences("bbcbaba".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_palindromic_subsequences("aabca".to_string()), 3);
    assert_eq!(count_palindromic_subsequences("adc".to_string()), 0);
    assert_eq!(count_palindromic_subsequences("bbcbaba".to_string()), 4);
}
