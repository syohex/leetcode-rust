fn count_hill_valley(nums: Vec<i32>) -> i32 {
    (1..nums.len() - 1)
        .filter(|&i| {
            if nums[i - 1] == nums[i] {
                return false;
            }

            let mut j = i - 1;
            let left = loop {
                if nums[j] != nums[i] {
                    break nums[j];
                }

                if j == 0 {
                    return false;
                }

                j -= 1;
            };

            j = i + 1;
            let right = loop {
                if nums[j] != nums[i] {
                    break nums[j];
                }

                if j == nums.len() - 1 {
                    return false;
                }

                j += 1;
            };

            (left > nums[i] && nums[i] < right) || (left < nums[i] && nums[i] > right)
        })
        .count() as i32
}

fn main() {
    let nums = vec![2, 4, 1, 1, 6, 5];
    let ret = count_hill_valley(nums);
    println!("ret={ret}");
}

#[test]
fn test_count_hill_valley() {
    {
        let nums = vec![2, 4, 1, 1, 6, 5];
        let ret = count_hill_valley(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![6, 6, 5, 5, 4, 1];
        let ret = count_hill_valley(nums);
        assert_eq!(ret, 0);
    }
}
