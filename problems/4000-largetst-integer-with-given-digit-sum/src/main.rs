fn largest_integer(n: i32, s: i32) -> i32 {
    if s > 9 * n {
        -1
    } else {
        let mut ret = 0;
        let mut s = s;
        let mut n = n;
        while s > 0 || n > 0 {
            let d = std::cmp::min(9, s);
            ret = 10 * ret + d;
            s -= d;
            n -= 1;
        }
        ret
    }
}

fn main() {
    let ret = largest_integer(2, 9);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_integer(2, 9), 90);
    assert_eq!(largest_integer(2, 19), -1);
    assert_eq!(largest_integer(5, 0), 0);
}
