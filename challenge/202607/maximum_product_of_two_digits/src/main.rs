fn max_product(n: i32) -> i32 {
    let mut first = i32::MIN;
    let mut second = i32::MIN;
    let mut n = n;

    while n > 0 {
        let d = n % 10;
        if d > first {
            (first, second) = (d, first);
        } else if d > second {
            second = d;
        }

        n /= 10;
    }

    first * second
}

fn main() {
    let ret = max_product(123456789);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_product(31), 3);
    assert_eq!(max_product(22), 4);
    assert_eq!(max_product(124), 8);
}
