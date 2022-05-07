fn find132pattern(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 3 {
        return false;
    }

    let mut min = std::i32::MAX;
    for i in 0..len {
        min = std::cmp::min(min, nums[i]);
        if min == nums[i] {
            continue;
        }

        for j in i + 1..len {
            if min < nums[j] && nums[j] < nums[i] {
                return true;
            }
        }
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
}
