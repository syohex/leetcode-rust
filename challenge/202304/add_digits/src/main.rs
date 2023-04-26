fn add_digits(num: i32) -> i32 {
    let mut n = num;
    loop {
        if n < 10 {
            return n;
        }

        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        n = sum;
    }
}

fn main() {
    let ret = add_digits(38);
    println!("ret={ret}");
}

#[test]
fn test_add_digits() {
    {
        let ret = add_digits(38);
        assert_eq!(ret, 2);
    }
    {
        let ret = add_digits(0);
        assert_eq!(ret, 0);
    }
}
