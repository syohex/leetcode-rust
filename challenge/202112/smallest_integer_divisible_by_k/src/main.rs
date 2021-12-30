fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut n = 0;
    for i in 1..=k {
        n = (n * 10 + 1) % k;
        if n == 0 {
            return i;
        }
    }

    -1
}

fn main() {
    println!("ret={}", smallest_repunit_div_by_k(3));
}

#[test]
fn test_smallest_repunit_div_by_k() {
    assert_eq!(smallest_repunit_div_by_k(1), 1);
    assert_eq!(smallest_repunit_div_by_k(2), -1);
    assert_eq!(smallest_repunit_div_by_k(3), 3);
}
