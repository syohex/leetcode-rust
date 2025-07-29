fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    let mut ret = vec![0; len];
    let mut bits = vec![usize::MAX; 32];
    for i in (0..len).rev() {
        let mut max_bit = i;
        for j in 0..32 {
            let b = nums[i] & (1 << j);
            if b != 0 {
                bits[j] = i;
            } else if bits[j] != usize::MAX {
                max_bit = std::cmp::max(max_bit, bits[j]);
            }
        }

        ret[i] = (max_bit - i + 1) as i32;
    }

    ret
}

fn main() {
    let nums = vec![1, 0, 2, 1, 3];
    let ret = smallest_subarrays(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 0, 2, 1, 3];
        let expected = vec![3, 3, 2, 2, 1];
        let ret = smallest_subarrays(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2];
        let expected = vec![2, 1];
        let ret = smallest_subarrays(nums);
        assert_eq!(ret, expected);
    }
}
