fn check_divisibility(n: i32) -> bool {
    let mut m = n;

    let mut sum = 0;
    let mut product = 1;
    while m > 0 {
        let k = m % 10;
        sum += k;
        product *= k;

        m /= 10;
    }

    n % (sum + product) == 0
}

fn main() {
    let ret = check_divisibility(99);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_divisibility(99));
    assert!(!check_divisibility(23));
}
