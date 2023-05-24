fn min_length(s: String) -> i32 {
    let mut v: Vec<char> = s.chars().collect();

    loop {
        let mut ok = true;
        let mut tmp = Vec::new();

        let mut i = 0;
        while i < v.len() {
            if i < v.len() - 1 && ((v[i] == 'A' && v[i + 1] == 'B') || (v[i] == 'C' && v[i + 1] == 'D')) {
                i += 2;
                ok = false;
            } else {
                tmp.push(v[i]);
                i += 1;
            }
        }

        if ok {
            return v.len() as i32;
        }

        v = tmp;
    }
}

fn main() {
    let ret = min_length("ABFCACDB".to_string());
    println!("ret={ret}");
}

#[test]
fn test_min_length() {
    assert_eq!(min_length("ABFCACDB".to_string()), 2);
    assert_eq!(min_length("ACBBD".to_string()), 5);
}
