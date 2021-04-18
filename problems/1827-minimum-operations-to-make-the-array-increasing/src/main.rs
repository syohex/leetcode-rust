fn min_operations(nums: Vec<i32>) -> i32 {
    let mut prev = nums[0];
    let mut count = 0;
    for n in nums.iter().skip(1) {
        if *n > prev {
            prev = *n;
        } else {
            count += prev + 1 - *n;
            prev = prev + 1;
        }
    }

    count
}

fn main() {
    let ret = min_operations(vec![1, 1, 1]);
    println!("ret={}", ret);
}

#[test]
fn test_min_operations() {
    {
        let nums = vec![1, 1, 1];
        assert_eq!(min_operations(nums), 3);
    }
    {
        let nums = vec![1, 5, 2, 4, 1];
        assert_eq!(min_operations(nums), 14);
    }
    {
        let nums = vec![8];
        assert_eq!(min_operations(nums), 0);
    }
}
