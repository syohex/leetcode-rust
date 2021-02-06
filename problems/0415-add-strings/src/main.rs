fn add_strings(num1: String, num2: String) -> String {
    let len = std::cmp::max(num1.len(), num2.len());
    let b1: Vec<u8> = num1.bytes().rev().collect();
    let b2: Vec<u8> = num2.bytes().rev().collect();

    let mut v: Vec<u8> = vec![];
    let mut carry = 0;
    for i in 0..len {
        let n1 = if i < num1.len() { b1[i] } else { b'0' };
        let n2 = if i < num2.len() { b2[i] } else { b'0' };

        let n = n1 - b'0' + n2 - b'0' + carry;
        if n >= 10 {
            v.push((n % 10) + b'0');
            carry = 1;
        } else {
            v.push(n + b'0');
            carry = 0;
        }
    }

    if carry != 0 {
        v.push(b'1');
    }

    v.reverse();
    String::from_utf8(v).unwrap()
}

fn main() {
    println!(
        "add_strings('123', '456')={}",
        add_strings("123".to_string(), "456".to_string())
    );
}

#[test]
fn test_add_string() {
    assert_eq!(add_strings("123".to_string(), "456".to_string()), "579");
    assert_eq!(add_strings("1".to_string(), "9".to_string()), "10");
    assert_eq!(add_strings("9999".to_string(), "1".to_string()), "10000");
}
