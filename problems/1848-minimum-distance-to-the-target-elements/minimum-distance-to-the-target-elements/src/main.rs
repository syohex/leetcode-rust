fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    use std::cmp::min;
    nums.iter().enumerate().fold(std::i32::MAX, |acc, (i, n)| {
        if *n == target {
            min(acc, (i as i32 - start).abs())
        } else {
            acc
        }
    })
}

fn main() {
    let ret = get_min_distance(vec![1, 2, 3, 4, 5], 5, 3);
    println!("ret = {}", ret);
}

#[test]
fn test_get_min_distance() {
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(get_min_distance(nums, 5, 3), 1);
    }
    {
        let nums = vec![1];
        assert_eq!(get_min_distance(nums, 1, 0), 0);
    }
    {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(get_min_distance(nums, 1, 0), 0);
    }
}
