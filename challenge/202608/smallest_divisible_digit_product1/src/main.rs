fn smallest_number(n: i32, t: i32) -> i32 {
    let mut n = n;
    loop {
        let mut m = n;
        let mut product = 1;
        while m > 0 {
            product *= m % 10;
            m /= 10;
        }

        if product % t == 0 {
            return n;
        }
        n += 1;
    }
}

fn main() {
    let ret = smallest_number(15, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_number(10, 2), 10);
    assert_eq!(smallest_number(15, 3), 16);
}
