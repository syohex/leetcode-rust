fn check_good_integer(n: i32) -> bool {
    let mut n = n;
    let mut digit_sum = 0;
    let mut square_sum = 0;

    while n > 0 {
        let m = n % 10;
        digit_sum += m;
        square_sum += m * m;
        n /= 10;
    }

    square_sum - digit_sum >= 50
}

fn main() {
    let ret = check_good_integer(1000);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!check_good_integer(1000));
    assert!(check_good_integer(19));
}
