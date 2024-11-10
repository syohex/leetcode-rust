fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    fn set_bit(n: i32, bits: &mut Vec<i32>) {
        for i in 0..32 {
            bits[i] += (n >> i) & 1;
        }
    }

    fn unset_bit(n: i32, bits: &mut Vec<i32>) {
        for i in 0..32 {
            bits[i] -= (n >> i) & 1;
        }
    }

    fn or_bit(bits: &Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..32 {
            ret |= if bits[i] == 0 { 0 } else { 1 << i }
        }

        ret
    }

    let mut left = 0;
    let mut right = 0;
    let mut bits = vec![0; 32];
    let mut ret = std::usize::MAX;

    while right < nums.len() {
        set_bit(nums[right], &mut bits);

        while left <= right && or_bit(&bits) >= k {
            ret = std::cmp::min(ret, right - left + 1);

            unset_bit(nums[left], &mut bits);
            left += 1;
        }

        right += 1;
    }

    if ret == std::usize::MAX {
        -1
    } else {
        ret as i32
    }
}

fn main() {
    let nums = vec![2, 1, 8];
    let ret = minimum_subarray_length(nums, 10);
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
}
