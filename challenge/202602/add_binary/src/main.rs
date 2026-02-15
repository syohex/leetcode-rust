fn add_binary(a: String, b: String) -> String {
    fn f(s: &str) -> Vec<i32> {
        s.chars()
            .rev()
            .map(|c| if c == '0' { 0 } else { 1 })
            .collect()
    }

    let a = f(&a);
    let b = f(&b);
    let len = std::cmp::max(a.len(), b.len());

    let mut ret = vec![];
    let mut carry = 0;
    for i in 0..len {
        let av = *a.get(i).unwrap_or(&0);
        let bv = *b.get(i).unwrap_or(&0);
        let v = av + bv + carry;
        if v >= 2 {
            ret.push(v % 2);
            carry = 1;
        } else {
            ret.push(v);
            carry = 0;
        }
    }

    if carry != 0 {
        ret.push(carry);
    }

    ret.into_iter()
        .rev()
        .map(|n| char::from_digit(n as u32, 10).unwrap())
        .collect()
}

fn main() {
    let ret = add_binary("111".to_string(), "1".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(add_binary("1111".to_string(), "1111".to_string()), "11110");
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
}
