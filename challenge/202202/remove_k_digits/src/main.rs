fn remove_kdigits(num: String, k: i32) -> String {
    let mut stack = vec![];
    let mut k = k;

    for b in num.bytes() {
        while !stack.is_empty() && k > 0 {
            if let Some(last) = stack.last() {
                if *last > b {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
        }

        stack.push(b);
    }

    for _ in 0..k {
        stack.pop();
    }

    let mut ret = vec![];
    let mut zeros = true;
    for &b in stack.iter() {
        if zeros && b == b'0' {
            continue;
        }

        zeros = false;
        ret.push(b);
    }

    if ret.is_empty() {
        return "0".to_string();
    }

    String::from_utf8(ret).unwrap()
}

fn main() {
    let num = "1432219".to_string();
    let ret = remove_kdigits(num, 3);
    println!("ret={ret}");
}

#[test]
fn test_remove_kdigits() {
    {
        let num = "1432219".to_string();
        let ret = remove_kdigits(num, 3);
        assert_eq!(ret, "1219".to_string());
    }
    {
        let num = "10200".to_string();
        let ret = remove_kdigits(num, 1);
        assert_eq!(ret, "200".to_string());
    }
    {
        let num = "10".to_string();
        let ret = remove_kdigits(num, 2);
        assert_eq!(ret, "0".to_string());
    }
    {
        let num = "000000000001".to_string();
        let ret = remove_kdigits(num, 1);
        assert_eq!(ret, "0".to_string());
    }
}
