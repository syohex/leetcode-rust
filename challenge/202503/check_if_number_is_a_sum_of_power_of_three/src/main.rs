fn check_powers_of_three(n: i32) -> bool {
    fn f(n: i32, pos: usize, powers: &[i32]) -> bool {
        if n == 0 {
            return true;
        }

        if n < 0 || pos >= powers.len() {
            return false;
        }

        for i in pos..powers.len() {
            if f(n - powers[i], i + 1, powers) {
                return true;
            }
        }

        false
    }

    let mut powers = vec![];
    let mut v = 1;
    while v <= 10_000_000 {
        powers.push(v);
        v *= 3;
    }

    f(n, 0, &powers)
}

fn main() {
    let ret = check_powers_of_three(6378022);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_powers_of_three(12));
    assert!(check_powers_of_three(91));
    assert!(!check_powers_of_three(21));
    assert!(check_powers_of_three(6378022));
}
