fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let k = k as i64;
    let mut sum = 0i64;
    let mut ret = std::i64::MAX;

    let mut q = BinaryHeap::new();
    q.push(Reverse((0i64, -1)));

    for i in 0..nums.len() {
        sum += nums[i] as i64;

        while !q.is_empty() {
            let Reverse((s, j)) = q.pop().unwrap();
            if sum - s < k {
                q.push(Reverse((s, j)));
                break;
            }

            ret = std::cmp::min(ret, (i as i32 - j) as i64);
        }

        q.push(Reverse((sum, i as i32)))
    }

    if ret == std::i64::MAX {
        -1
    } else {
        ret as i32
    }
}

fn main() {
    let nums = vec![2, -1, 2];
    let ret = shortest_subarray(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1];
        let ret = shortest_subarray(nums, 1);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 2];
        let ret = shortest_subarray(nums, 4);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![2, -1, 2];
        let ret = shortest_subarray(nums, 3);
        assert_eq!(ret, 3);
    }
}
