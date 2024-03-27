fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;

    let mut ret = 0;
    let mut product = 1;
    while left < nums.len() && right < nums.len() {
        product *= nums[right];

        while product >= k && left < right {
            product /= nums[left];
            left += 1;
        }

        if product < k {
            ret += right - left + 1;
        }
        right += 1;
    }

    ret as i32
}

fn main() {
    let nums = vec![10, 5, 2, 6];
    let ret = num_subarray_product_less_than_k(nums, 100);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 5, 2, 6];
        let ret = num_subarray_product_less_than_k(nums, 100);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = num_subarray_product_less_than_k(nums, 0);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = num_subarray_product_less_than_k(nums, 1);
        assert_eq!(ret, 0);
    }
}
