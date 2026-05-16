fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut mins = vec![i32::MAX; len];
    mins[len - 1] = nums[len - 1];

    for (i, n) in nums.iter().enumerate().rev().skip(1) {
        mins[i] = std::cmp::min(*n, mins[i + 1]);
    }

    let mut max = -1;
    for (i, n) in nums.into_iter().enumerate() {
        max = std::cmp::max(max, n);
        let v = max - mins[i];
        if v <= k {
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
    assert_eq!(first_stable_index(vec![0], 0), 0);
}
