fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let k = k as usize;
    let mut q = VecDeque::new();

    for i in 0..k {
        while !q.is_empty() && nums[i] >= nums[*q.back().unwrap()] {
            q.pop_back();
        }

        q.push_back(i);
    }

    let mut ret = vec![];
    ret.push(nums[*q.front().unwrap()]);

    for i in k..nums.len() {
        if *q.front().unwrap() == i - k {
            q.pop_front();
        }

        while !q.is_empty() && nums[i] >= nums[*q.back().unwrap()] {
            q.pop_back();
        }

        q.push_back(i);
        ret.push(nums[*q.front().unwrap()]);
    }

    ret
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let ret = max_sliding_window(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test_max_sliding_window() {
    {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let expected = vec![3, 3, 5, 5, 6, 7];
        let ret = max_sliding_window(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1];
        let expected = vec![1];
        let ret = max_sliding_window(nums, 1);
        assert_eq!(ret, expected);
    }
}
