fn three_fibonacci_sum(n: i32) -> bool {
    let mut v = vec![0, 1, 1];

    loop {
        let len = v.len();
        let sum = v[len - 1] + v[len - 2] + v[len - 3];
        if sum == n {
            return true;
        }
        if v[len - 1] > n {
            return false;
        }

        v.push(v[len - 1] + v[len - 2]);
    }
}

fn main() {
    let ret = three_fibonacci_sum(26);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!three_fibonacci_sum(5));
    assert!(three_fibonacci_sum(16));
    assert!(!three_fibonacci_sum(8));
    assert!(three_fibonacci_sum(16));
}
