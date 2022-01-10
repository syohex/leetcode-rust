fn add_biary(a: String, b: String) -> String {
    let av: Vec<u8> = a.bytes().rev().map(|u| u - b'0').collect();
    let bv: Vec<u8> = b.bytes().rev().map(|u| u - b'0').collect();

    let mut ret = Vec::new();
    let mut i = 0;
    let mut carry = 0;
    let mut finish = false;
    while !finish {
        match (av.get(i), bv.get(i)) {
            (None, None) => {
                finish = true;
            }
            (Some(m), None) => {
                let mut v = *m + carry;
                if v == 2 {
                    v = 0;
                    carry = 1;
                } else {
                    carry = 0;
                }
                ret.push(v + b'0');
            }
            (None, Some(m)) => {
                let mut v = *m + carry;
                if v == 2 {
                    v = 0;
                    carry = 1;
                } else {
                    carry = 0;
                }
                ret.push(v + b'0');
            }
            (Some(m), Some(n)) => {
                let mut v = *m + *n + carry;
                if v >= 2 {
                    v %= 2;
                    carry = 1;
                } else {
                    carry = 0;
                }
                ret.push(v + b'0');
            }
        }

        i += 1;
    }

    if carry != 0 {
        ret.push(carry + b'0')
    }

    String::from_utf8(ret.into_iter().rev().collect()).unwrap()
}

fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    let ret = add_biary(a, b);
    println!("ret={}", ret);
}

#[test]
fn test_add_binary() {
    {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!(add_biary(a, b), "100".to_string());
    }
    {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!(add_biary(a, b), "10101".to_string());
    }
}
