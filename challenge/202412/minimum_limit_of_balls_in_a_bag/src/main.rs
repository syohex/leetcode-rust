fn minimize_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    fn f(balls: i32, nums: &Vec<i32>, max_operations: i32) -> bool {
        let mut operations = 0;
        for num in nums {
            let count = (*num as f64 / balls as f64).ceil() as i32 - 1;
            operations += count;
            if operations > max_operations {
                return false;
            }
        }
        true
    }

    let mut left = 0;
    let mut right = *nums.iter().max().unwrap();

    while left < right {
        let mid = left + (right - left) / 2;
        if f(mid, &nums, max_operations) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    let nums = vec![9];
    let ret = minimize_size(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![9];
        let ret = minimize_size(nums, 2);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![2, 4, 8, 2];
        let ret = minimize_size(nums, 4);
        assert_eq!(ret, 2);
    }
}
