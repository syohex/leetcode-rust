fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let len = nums.len();
    let mut acc = vec![0; len + 1];
    for (i, n) in nums.into_iter().enumerate() {
        acc[i + 1] = acc[i] + n;
    }

    let mut ret = i32::MAX;
    for i in (l as usize)..=len {
        let end = i - l as usize;
        let start = std::cmp::max(0, i as i32 - r);
        for j in (start as usize)..=end {
            let diff = acc[i] - acc[j];
            if diff > 0 {
                ret = std::cmp::min(ret, diff);
            }
        }
    }

    if ret == i32::MAX { -1 } else { ret }
}

fn main() {
    let ret = minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
    assert_eq!(minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
    assert_eq!(minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
}
