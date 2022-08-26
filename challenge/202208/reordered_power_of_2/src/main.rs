fn reordered_power_of2(n: i32) -> bool {
    fn to_digits_vec(n: i32) -> Vec<i32> {
        let mut v = vec![];
        let mut n = n;

        while n > 0 {
            v.push(n % 10);
            n /= 10;
        }

        v.sort_unstable();
        v
    }

    let n_digits = to_digits_vec(n);
    (0..31)
        .into_iter()
        .map(|m| to_digits_vec(2_i32.pow(m)))
        .find(|v| n_digits == *v)
        .is_some()
}

fn main() {
    let ret = reordered_power_of2(56635);
    println!("ret={ret}");
}

#[test]
fn test_reordered_power_of2() {
    // assert!(reordered_power_of2(1));
    assert!(!reordered_power_of2(10));
    assert!(reordered_power_of2(56635));
}
