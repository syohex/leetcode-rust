fn clear_stars(s: String) -> String {
    let mut bs: Vec<_> = s.bytes().collect();
    let mut counts = vec![vec![]; 26];
    for i in 0..bs.len() {
        if bs[i] != b'*' {
            counts[(bs[i] - b'a') as usize].push(i);
        } else {
            for j in 0..26 {
                if let Some(idx) = counts[j].pop() {
                    bs[idx] = b'*';
                    break;
                }
            }
        }
    }

    let mut ret = String::new();
    for b in bs {
        if b != b'*' {
            ret.push(b as char);
        }
    }

    ret
}

fn main() {
    let s = "aaba*".to_string();
    let ret = clear_stars(s);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "aaba*".to_string();
        let ret = clear_stars(s);
        assert_eq!(ret, "aab");
    }
    {
        let s = "abc".to_string();
        let ret = clear_stars(s);
        assert_eq!(ret, "abc");
    }
    {
        let s = "d*c".to_string();
        let ret = clear_stars(s);
        assert_eq!(ret, "c");
    }
}
