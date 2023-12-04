fn largest_good_number(num: String) -> String {
    let cs: Vec<u8> = num.bytes().collect();
    let mut ret = "".to_string();
    let mut max = -1;
    for i in 0..(cs.len() - 2) {
        if cs[i] == cs[i + 1] && cs[i + 1] == cs[i + 2] {
            let n = (cs[i] - b'0') as i32;
            let val = 100 * n + 10 * n + n;
            if val > max {
                max = val;
                ret = format!("{}{}{}", n, n, n);
            }
        }
    }

    ret
}

fn main() {
    let num = "6777133339".to_string();
    let ret = largest_good_number(num);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let num = "6777133339".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "777");
    }
    {
        let num = "2300019".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "000");
    }
    {
        let num = "42352338".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "");
    }
    {
        let num = "111456".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "111");
    }
    {
        let num = "012345678999".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "999");
    }
    {
        let num = "3200014888".to_string();
        let ret = largest_good_number(num);
        assert_eq!(ret, "888");
    }
}
