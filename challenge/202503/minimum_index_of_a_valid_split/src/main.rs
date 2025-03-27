fn minimum_index(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let (k, v) = nums
        .iter()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .fold((0, 0), |(max_key, max_val), (k, v)| {
            if v > max_val {
                (*k, v)
            } else {
                (max_key, max_val)
            }
        });

    let len = nums.len();
    let mut left_count = 0;
    let mut right_count = v;
    for i in 0..(len - 1) {
        if nums[i] == k {
            left_count += 1;
            right_count -= 1;
        }

        let left_size = i + 1;
        let right_size = len - i - 1;

        if left_count > left_size / 2 && right_count > right_size / 2 {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let nums = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
    let ret = minimum_index(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 2, 2];
        let ret = minimum_index(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
        let ret = minimum_index(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![3, 3, 3, 3, 7, 2, 2];
        let ret = minimum_index(nums);
        assert_eq!(ret, -1);
    }
}
