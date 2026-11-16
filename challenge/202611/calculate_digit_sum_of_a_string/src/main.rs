fn digit_sum(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s = s;

    loop {
        if s.len() <= k {
            return s;
        }

        let mut v: Vec<_> = s.bytes().map(|b| (b - b'0') as i32).collect();
        v = v.chunks(k).map(|nums| nums.iter().sum()).collect();
        s = v.into_iter().fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        });
    }
}

fn main() {
    let ret = digit_sum("01234567890".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(digit_sum("01234567890".to_string(), 2), "27");
    assert_eq!(digit_sum("11111222223".to_string(), 3), "135");
    assert_eq!(digit_sum("00000000".to_string(), 3), "000");
}
