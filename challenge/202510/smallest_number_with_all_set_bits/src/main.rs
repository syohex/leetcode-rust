fn smallest_number(n: i32) -> i32 {
    let mut m = 2;
    loop {
        if m > n {
            return m - 1;
        }

        m <<= 1;
    }
}

fn main() {
    let ret = smallest_number(19284);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_number(4), 7);
    assert_eq!(smallest_number(1), 1);
    assert_eq!(smallest_number(5), 7);
    assert_eq!(smallest_number(10), 15);
    assert_eq!(smallest_number(3), 3);
}
