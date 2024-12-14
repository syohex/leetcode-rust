fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut min_q = BinaryHeap::new();
    let mut max_q = BinaryHeap::new();

    let mut ret = 0i64;
    let mut left = 0;
    for (right, n) in nums.into_iter().enumerate() {
        min_q.push(Reverse((n, right)));
        max_q.push((n, Reverse(right)));

        while left < right {
            let min = min_q.peek().unwrap().0 .0;
            let max = max_q.peek().unwrap().0;

            if max - min <= 2 {
                break;
            }
            left += 1;
            while let Some(Reverse((_, i))) = min_q.peek() {
                if *i < left {
                    min_q.pop();
                    continue;
                }

                break;
            }
            while let Some((_, Reverse(i))) = max_q.peek() {
                if *i < left {
                    max_q.pop();
                    continue;
                }
                break;
            }
        }

        ret += (right - left + 1) as i64;
    }

    ret
}

fn main() {
    let nums = vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64];
    let ret = continuous_subarrays(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 4, 2, 4];
        let ret = continuous_subarrays(nums);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = continuous_subarrays(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64];
        let ret = continuous_subarrays(nums);
        assert_eq!(ret, 43);
    }
    {
        let nums = vec![42, 41, 42, 41, 41, 40, 39, 38];
        let ret = continuous_subarrays(nums);
        assert_eq!(ret, 28);
    }
    {
        let nums = vec![35, 35, 36, 37, 36, 37, 38, 37, 38];
        let ret = continuous_subarrays(nums);
        assert_eq!(ret, 39);
    }
}
