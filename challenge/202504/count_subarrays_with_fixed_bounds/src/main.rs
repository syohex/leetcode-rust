fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut min_pos = -1;
    let mut max_pos = -1;
    let mut left = -1;

    let mut ret = 0;
    for (i, n) in nums.into_iter().enumerate() {
        if n < min_k || n > max_k {
            left = i as i32;
        }

        if n == min_k {
            min_pos = i as i32;
        }
        if n == max_k {
            max_pos = i as i32;
        }

        let min_pos = std::cmp::min(min_pos, max_pos);
        if min_pos >= left {
            ret += (min_pos - left) as i64;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 1, 1, 1];
    let ret = count_subarrays(nums, 1, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let ret = count_subarrays(nums, 1, 5);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 1, 1, 1];
        let ret = count_subarrays(nums, 1, 1);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![4,3];
        let ret = count_subarrays(nums, 3, 3);
        assert_eq!(ret, 1);
    }
}
