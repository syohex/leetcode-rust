fn remove_kdigits(num: String, k: i32) -> String {
    let mut stack = vec![];
    let mut k = k;

    for c in num.chars() {
        while !stack.is_empty() && k > 0 {
            if let Some(last) = stack.last() {
                if *last > c {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
        }

        stack.push(c);
    }

    for _ in 0..k {
        stack.pop();
    }

    let mut ret = String::new();
    for c in stack.into_iter().skip_while(|c| *c == '0') {
        ret.push(c);
    }

    if ret.is_empty() {
        "0".to_string()
    } else {
        ret
    }
}

fn main() {
    let ret = remove_kdigits("1432219".to_string(), 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(remove_kdigits("1432219".to_string(), 3), "1219");
    assert_eq!(remove_kdigits("10200".to_string(), 1), "200");
    assert_eq!(remove_kdigits("10".to_string(), 2), "0");
}
