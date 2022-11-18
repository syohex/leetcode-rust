fn is_ugly(n: i32) -> bool {
    fn f(mut n: i32, div: i32) -> i32 {
        loop {
            if n % div != 0 {
                return n;
            }

            n /= div;
        }
    }

    if n <= 0 {
        return false;
    }

    let n = f(n, 2);
    let n = f(n, 3);
    let n = f(n, 5);
    n == 1
}

fn main() {
    println!(
        "is_ugly(6, 1, 14) = {}, {}, {}",
        is_ugly(6),
        is_ugly(1),
        is_ugly(14)
    );
}

#[test]
fn test_is_ugly() {
    assert!(is_ugly(6));
    assert!(is_ugly(1));
    assert!(!is_ugly(14));
    assert!(!is_ugly(0));
    assert!(!is_ugly(-100));
}
