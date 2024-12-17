fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    use std::collections::BinaryHeap;

    let mut v = vec![0; 26];
    for b in s.bytes() {
        let index = (b - b'a') as usize;
        v[index] += 1;
    }

    let mut q = BinaryHeap::new();
    for (i, count) in v.into_iter().enumerate() {
        if count > 0 {
            let c = (i as u8 + b'a') as char;
            q.push((c, count));
        }
    }

    let mut ret = String::new();
    while let Some((c, count)) = q.pop() {
        for _ in 0..std::cmp::min(count, repeat_limit) {
            ret.push(c);
        }

        if count > repeat_limit {
            if let Some((c2, count2)) = q.pop() {
                ret.push(c2);
                if count2 > 1 {
                    q.push((c2, count2 - 1));
                }
                q.push((c, count - repeat_limit));
            } else {
                break;
            }
        }
    }

    ret
}

fn main() {
    let s = "aababab".to_string();
    let ret = repeat_limited_string(s, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "cczazcc".to_string();
        let ret = repeat_limited_string(s, 3);
        assert_eq!(ret, "zzcccac");
    }
    {
        let s = "aababab".to_string();
        let ret = repeat_limited_string(s, 2);
        assert_eq!(ret, "bbabaa");
    }
}
