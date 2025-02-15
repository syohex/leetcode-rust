fn punishment_number(n: i32) -> i32 {
    fn is_punishment(n: i32, m: i32) -> bool {
        if m < 0 || n < m {
            return false;
        }

        if n == m {
            return true;
        }

        is_punishment(n / 10, m - n % 10)
            || is_punishment(n / 100, m - n % 100)
            || is_punishment(n / 1000, m - n % 1000)
    }

    (1..=n).fold(0, |acc, n| {
        if is_punishment(n * n, n) {
            acc + n * n
        } else {
            acc
        }
    })
}

fn main() {
    let ret = punishment_number(37);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(punishment_number(10), 182);
    assert_eq!(punishment_number(37), 1478);
}
