fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    arr1.into_iter()
        .filter(|n| arr2.iter().all(|m| (n - *m).abs() > d))
        .count() as i32
}

fn main() {
    let ret = find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
        2
    );
    assert_eq!(
        find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
    assert_eq!(
        find_the_distance_value(vec![2, 1, 100], vec![-5, -2, 10, -3, 7], 6),
        1
    );
}
