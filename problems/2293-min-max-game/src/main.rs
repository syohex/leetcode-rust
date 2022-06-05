fn min_max_game(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    while nums.len() > 1 {
        let half = nums.len() / 2;

        for i in 0..half {
            if i % 2 == 0 {
                nums[i] = std::cmp::min(nums[i * 2], nums[i * 2 + 1]);
            } else {
                nums[i] = std::cmp::max(nums[i * 2], nums[i * 2 + 1]);
            }
        }

        nums.resize(half, 0);
    }

    nums[0]
}

fn main() {
    let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
    let ret = min_max_game(nums);
    println!("ret={ret}");
}

#[test]
fn test_min_max_game() {
    {
        let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
        let ret = min_max_game(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![3];
        let ret = min_max_game(nums);
        assert_eq!(ret, 3);
    }
}
