fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut ret = sum;
    for (i, &n) in nums.iter().enumerate().skip(1) {
        if nums[i - 1] < n {
            sum += n;
        } else {
            sum = n;
        }

        ret = std::cmp::max(ret, sum);
    }
    ret
}

fn main() {
    let nums = vec![10, 20, 30, 5, 10, 50];
    let ret = max_ascending_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 20, 30, 5, 10, 50];
        let ret = max_ascending_sum(nums);
        assert_eq!(ret, 65);
    }
    {
        let nums = vec![10, 20, 30, 40, 50];
        let ret = max_ascending_sum(nums);
        assert_eq!(ret, 150);
    }
    {
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        let ret = max_ascending_sum(nums);
        assert_eq!(ret, 33);
    }
}
