fn count_digits(num: i32) -> i32 {
    num.to_string()
        .bytes()
        .map(|b| (b - b'0') as i32)
        .filter(|&n| num % n == 0)
        .count() as i32
}

fn main() {
    let ret = count_digits(121);
    println!("ret={ret}");
}

#[test]
fn test_count_digits() {
    {
        let ret = count_digits(7);
        assert_eq!(ret, 1);
    }
    {
        let ret = count_digits(121);
        assert_eq!(ret, 2);
    }
    {
        let ret = count_digits(1248);
        assert_eq!(ret, 4);
    }
}
