fn number_of_matches(n: i32) -> i32 {
    let mut n = n;
    let mut ret = 0;
    let mut is_even = true;
    loop {
        let matches = if is_even { n / 2 } else { (n - 1) / 2 };
        ret += matches;
        n -= matches;

        if n == 1 {
            return ret;
        }

        is_even = !is_even;
    }
}

fn main() {
    println!("ret={}", number_of_matches(14));
}

#[test]
fn test() {
    assert_eq!(number_of_matches(7), 6);
    assert_eq!(number_of_matches(14), 13);
}
