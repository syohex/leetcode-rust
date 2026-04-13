fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    nums.into_iter()
        .enumerate()
        .filter(|(_, n)| *n == target)
        .fold(i32::MAX, |acc, (i, _)| {
            std::cmp::min(acc, (i as i32 - start).abs())
        })
}

fn main() {
    let ret = get_min_distance(vec![1, 2, 3, 4, 5], 5, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    assert_eq!(get_min_distance(vec![1], 1, 0), 0);
    assert_eq!(get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0), 0);
}
