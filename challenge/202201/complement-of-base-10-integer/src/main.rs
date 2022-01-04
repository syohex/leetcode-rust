fn bitwise_complement(n: i32) -> i32 {
    let mut n = n;
    let mut v = Vec::<i32>::new();

    if n == 0 {
        v.push(0);
    } else {
        while n > 0 {
            v.push(n % 2);
            n /= 2;
        }
    }

    v.into_iter()
        .rev()
        .fold(0, |acc, n| if n == 0 { acc * 2 + 1 } else { acc * 2 })
}

fn main() {
    println!("ret={}", bitwise_complement(5));
    println!("ret={}", bitwise_complement(7));
    println!("ret={}", bitwise_complement(10));
}

#[test]
fn test_bitwise_complement() {
    assert_eq!(bitwise_complement(5), 2);
    assert_eq!(bitwise_complement(7), 0);
    assert_eq!(bitwise_complement(10), 5);
}
