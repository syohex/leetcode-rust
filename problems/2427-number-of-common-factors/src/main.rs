fn common_factors(a: i32, b: i32) -> i32 {
    (1..=(std::cmp::min(a, b)))
    .into_iter()
    .fold(0, |acc, i| {
        if a % i == 0 && b % i == 0 {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let ret = common_factors(12, 6);
    println!("ret={ret}");
}

#[test]
fn test_common_factors() {
    assert_eq!(common_factors(12, 6), 4);
    assert_eq!(common_factors(25, 30), 2);
}
