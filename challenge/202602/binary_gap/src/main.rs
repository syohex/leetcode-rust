fn binary_gap(n: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut ret = 0;
    for i in 0..32 {
        if (n & (1 << i)) != 0 {
            if prev != i32::MIN {
                ret = std::cmp::max(ret, i - prev);
            }

            prev = i;
        }
    }

    ret
}

fn main() {
    let ret = binary_gap(22);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(binary_gap(343), 2);
    assert_eq!(binary_gap(22), 2);
    assert_eq!(binary_gap(8), 0);
    assert_eq!(binary_gap(5), 2);
}
