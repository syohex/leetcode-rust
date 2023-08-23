fn reorganize_string(s: String) -> String {
    use std::collections::BinaryHeap;

    let mut table = vec![0; 26];
    for b in s.bytes() {
        table[(b - b'a') as usize] += 1;
    }

    let mut bq = BinaryHeap::new();
    for i in 0..26 {
        if table[i] > 0 {
            bq.push((table[i], (i as u8 + b'a') as char));
        }
    }

    let mut ret = String::new();
    while let Some((count, c)) = bq.pop() {
        if ret.is_empty() || c != ret.chars().last().unwrap() {
            ret.push(c);
            if count - 1 >= 1 {
                bq.push((count - 1, c));
            }
        } else {
            if let Some((count2, c2)) = bq.pop() {
                ret.push(c2);
                if count2 - 1 >= 1 {
                    bq.push((count2 - 1, c2));
                }

                bq.push((count, c));
            } else {
                return "".to_string();
            }
        }
    }

    ret
}

fn main() {
    let s = "aab".to_string();
    let ret = reorganize_string(s);
    println!("ret={ret}");
}

#[test]
fn test_reorganize_string() {
    {
        let s = "aab".to_string();
        let ret = reorganize_string(s);
        assert_eq!(ret, "aba");
    }
    {
        let s = "aaab".to_string();
        let ret = reorganize_string(s);
        assert_eq!(ret, "");
    }
}
