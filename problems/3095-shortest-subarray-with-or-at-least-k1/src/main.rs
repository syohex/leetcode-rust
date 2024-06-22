fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    for i in 1..=nums.len() {
        for j in 0..=(nums.len() - i) {
            let mut acc = 0;
            for k in j..(j + i) {
                acc |= nums[k];
            }

            if acc >= k {
                return i as i32;
            }
        }
    }

    -1
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = minimum_subarray_length(nums, 10000);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3];
        let ret = minimum_subarray_length(nums, 2);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![2, 1, 8];
        let ret = minimum_subarray_length(nums, 10);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 2];
        let ret = minimum_subarray_length(nums, 0);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![32, 2, 24, 1];
        let ret = minimum_subarray_length(nums, 35);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1,12,2,5];
        let ret = minimum_subarray_length(nums, 43);
        assert_eq!(ret, -1);
    }
}
