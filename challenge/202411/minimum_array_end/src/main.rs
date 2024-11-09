fn min_end(n: i32, x: i32) -> i64 {
    fn to_bits(n: i32) -> Vec<i32> {
        let mut ret = vec![0; 64];
        let mut n = n;
        for v in ret.iter_mut() {
            *v = n & 1;
            n /= 2;
        }

        ret
    }

    let x_bits = to_bits(x);
    let n_bits = to_bits(n - 1);

    let mut tmp = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < x_bits.len() || j < n_bits.len() {
        if i < x_bits.len() && x_bits[i] == 1 {
            i += 1;
            tmp.push(1);
            continue;
        }

        tmp.push(n_bits[j]);
        i += 1;
        j += 1;
    }

    tmp.into_iter().rev().skip_while(|n| *n == 0).fold(0i64, |acc, n| {
        acc * 2 + n as i64
    })
}

fn main() {
    let ret = min_end(2, 7);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_end(3, 4), 6);
    assert_eq!(min_end(2, 7), 15);
}
