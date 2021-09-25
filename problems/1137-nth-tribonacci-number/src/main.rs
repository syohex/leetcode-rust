fn tribonacchi(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        n if n >= 3 => {
            let mut v0 = 0;
            let mut v1 = 1;
            let mut v2 = 1;
            let mut ret = 0;
            for _ in 3..=n {
                ret = v0 + v1 + v2;
                v0 = v1;
                v1 = v2;
                v2 = ret;
            }

            ret
        }
        _ => panic!("negative number is invalid"),
    }
}

fn main() {
    println!("ret={}", tribonacchi(25));
}

#[test]
fn test_tribonacchi() {
    assert_eq!(tribonacchi(4), 4);
    assert_eq!(tribonacchi(25), 1389537);
}
