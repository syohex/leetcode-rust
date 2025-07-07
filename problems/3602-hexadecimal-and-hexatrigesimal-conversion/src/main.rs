fn concat_hex36(n: i32) -> String {
    fn f(n: i32, base: i32) -> String {
        let mut v = vec![];
        let mut n = n;

        while n > 0 {
            let m = n % base;
            if m < 10 {
                v.push((m as u8 + b'0') as char);
            } else {
                v.push(((m - 10) as u8 + b'A') as char);
            }

            n /= base;
        }

        v.reverse();
        v.into_iter().collect()
    }

    let double = n * n;
    let triple = n * n * n;

    let s1 = f(double, 16);
    let s2 = f(triple, 36);

    format!("{s1}{s2}")
}

fn main() {
    let ret = concat_hex36(36);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(concat_hex36(13), "A91P1");
    assert_eq!(concat_hex36(36), "5101000");
}
