fn sum_and_multiply(n: i32) -> i64 {
    let (num, sum) = n.to_string().bytes().fold((0i64, 0i64), |(num, sum), b| {
        if b == b'0' {
            (num ,sum)
        } else {
            let m = (b - b'0') as i64;
            (num * 10 + m, sum + m)
        }
    });

    num * sum
}

fn main() {
    let ret = sum_and_multiply(10203004);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_and_multiply(10203004), 12340);
    assert_eq!(sum_and_multiply(1000), 1);
}
