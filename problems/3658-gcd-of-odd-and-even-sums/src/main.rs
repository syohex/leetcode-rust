fn gcd_of_odd_even_sums(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 2;

    for i in 1..n {
        a += 1 + 2 * i;
        b += 2 + 2 * i;
    }

    loop {
        let m = b % a;
        if m == 0 {
            return a;
        }

        b = a;
        a = m;
    }
}

fn main() {
    let ret = gcd_of_odd_even_sums(100);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(gcd_of_odd_even_sums(4), 4);
    assert_eq!(gcd_of_odd_even_sums(5), 5);
}
