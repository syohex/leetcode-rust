fn count_commas(n: i32) -> i32 {
    std::cmp::max(0, n - 1000 + 1)
}

fn main() {
    let ret = count_commas(1002);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_commas(1002), 3);
    assert_eq!(count_commas(998), 0);
}
