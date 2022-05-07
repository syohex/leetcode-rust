fn find132pattern(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 3 {
        return false;
    }

    let mut mins = vec![0; len];
    mins[0] = nums[0];
    for i in 1..len {
        mins[i] = std::cmp::min(mins[i - 1], nums[i]);
    }

    let mut stack = vec![];
    for i in (1..len).rev() {
        while !stack.is_empty() && *stack.last().unwrap() <= mins[i] {
            stack.pop();
        }

        if !stack.is_empty() && *stack.last().unwrap() < nums[i] {
            return true;
        }

        stack.push(nums[i]);
    }

    false
}

fn main() {
    let nums = vec![3, 1, 4, 2];
    println!("ret={}", find132pattern(nums));
}

#[test]
fn test_find132pattern() {
    {
        let nums = vec![1, 2, 3, 4];
        assert!(!find132pattern(nums));
    }
    {
        let nums = vec![3, 1, 4, 2];
        assert!(find132pattern(nums));
    }
    {
        let nums = vec![-1, 3, 2, 0];
        assert!(find132pattern(nums));
    }
    {
        let nums = vec![-2, 1, -1];
        assert!(find132pattern(nums));
    }
}
