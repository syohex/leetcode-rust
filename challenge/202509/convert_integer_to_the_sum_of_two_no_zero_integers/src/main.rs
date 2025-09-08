fn get_no_zero_integers(n: i32) -> Vec<i32> {
    fn is_non_zero_integer(n: i32) -> bool {
        !n.to_string().contains(['0'])
    }

    for a in 1..n {
        let b = n - a;
        if is_non_zero_integer(a) && is_non_zero_integer(b) {
            return vec![a, b];
        }
    }

    unreachable!();
}

fn main() {
    let ret = get_no_zero_integers(9847);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(get_no_zero_integers(11), vec![2, 9]);
}
