fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut acc: Vec<i32> = vec![0; nums.len() + 1];
    let mut ret = 0;
    let mut start = 0;
    let mut h: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        acc[i + 1] = acc[i] + nums[i];
        if let Some(p) = h.get(&nums[i]) {
            start = max(start, *p + 1);
        }

        h.insert(nums[i], i);
        ret = max(ret, acc[i + 1] - acc[start]);
    }

    ret
}

fn main() {
    let ret = maximum_unique_subarray(vec![4, 2, 4, 5, 6]);
    println!("ret={}", ret);
}

#[test]
fn test_maximum_unique_subarray() {
    assert_eq!(maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
}
