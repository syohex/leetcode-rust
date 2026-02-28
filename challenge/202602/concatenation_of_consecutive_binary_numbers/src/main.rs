fn concatenated_binary(n: i32) -> i32 {
    let modulo = 1_000_000_007;
    let mut ret = 0;
    for i in 1..=n {
        let mut bits = vec![];
        let mut tmp = i;
        while tmp > 0 {
            bits.push(tmp % 2);
            tmp /= 2;
        }

        for b in bits.into_iter().rev() {
            ret = (ret * 2 + b) % modulo;
        }
    }

    ret
}

fn main() {
    let ret = concatenated_binary(12);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(concatenated_binary(1), 1);
    assert_eq!(concatenated_binary(3), 27);
    assert_eq!(concatenated_binary(12), 505379714);
}
