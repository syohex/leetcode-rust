fn sum_zero(n: i32) -> Vec<i32> {
    let mut ret = vec![];
    let m = n / 2;
    for i in 1..=m {
        ret.push(i);
        ret.push(-i);
    }

    if n % 2 != 0 {
        ret.push(0);
    }

    ret
}

fn main() {
    let ret = sum_zero(10);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(sum_zero(6), vec![1, -1, 2, -2, 3, -3]);
    assert_eq!(sum_zero(5), vec![1, -1, 2, -2, 0]);
    assert_eq!(sum_zero(3), vec![1, -1, 0]);
    assert_eq!(sum_zero(1), vec![0]);
}
