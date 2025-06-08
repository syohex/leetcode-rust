fn lexical_order(n: i32) -> Vec<i32> {
    let mut ret = vec![];
    let mut v = 1;
    for _ in 0..n {
        ret.push(v);
        if v * 10 <= n {
            v *= 10;
        } else {
            while v >= n || v % 10 == 9 {
                v /= 10;
            }
            v += 1;
        }
    }

    ret
}
fn main() {
    let ret = lexical_order(13);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let ret = lexical_order(13);
        let expected = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(ret, expected);
    }
    {
        let ret = lexical_order(2);
        let expected = vec![1, 2];
        assert_eq!(ret, expected);
    }
}
