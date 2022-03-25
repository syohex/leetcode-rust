fn sum_of_three(num: i64) -> Vec<i64> {
    if num % 3 == 0 {
        let n = num / 3;
        vec![n - 1, n, n + 1]
    } else {
        vec![]
    }
}

fn main() {
    let ret = sum_of_three(33);
    println!("ret={:?}", ret);
}

#[test]
fn test_sum_of_three() {
    assert_eq!(sum_of_three(33), vec![10, 11, 12]);
    assert_eq!(sum_of_three(4), vec![]);
}
