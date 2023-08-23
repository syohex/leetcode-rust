fn reorganize_string(s: String) -> String {
    fn f(prev: usize, len: usize, acc: &mut String, table: &mut Vec<usize>) -> bool {
        if acc.len() == len {
            return true;
        }

        for i in 0..26 {
            if table[i] == 0 || i == prev {
                continue;
            }

            acc.push((i as u8 + b'a') as char);
            table[i] -= 1;
            if f(i, len, acc, table) {
                return true;
            }
            table[i] += 1;
            acc.pop();
        }

        false
    }

    let mut table = vec![0; 26];
    for b in s.bytes() {
        table[(b - b'a') as usize] += 1;
    }

    let mut acc = String::new();
    if f(27, s.len(), &mut acc, &mut table) {
        acc
    } else {
        "".to_string()
    }
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
