fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let mut mins = vec![i32::MAX; nums.len()];
    let mut min_val = i32::MAX;
    for (i, num) in nums.iter().enumerate().rev() {
        min_val = std::cmp::min(min_val, *num);
        mins[i] = min_val;
    }

    let mut max_val = i32::MIN;
    for (i, num) in nums.into_iter().enumerate() {
        max_val = std::cmp::max(max_val, num);
        if max_val - mins[i] <= k {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let ret = first_stable_index(vec![5, 0, 1, 4], 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(first_stable_index(vec![5, 0, 1, 4], 3), 3);
    assert_eq!(first_stable_index(vec![3, 2, 1], 1), -1);
}
