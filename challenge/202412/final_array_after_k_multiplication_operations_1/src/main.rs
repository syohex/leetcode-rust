fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let len = nums.len();
    let mut q = BinaryHeap::new();
    for (i, n) in nums.into_iter().enumerate() {
        q.push(Reverse((n, i)));
    }

    for _ in 0..k {
        if let Some(Reverse((n, i))) = q.pop() {
            q.push(Reverse((n * multiplier, i)));
        }
    }

    let mut ret = vec![0;len];
    while let Some(Reverse((n, i))) = q.pop() {
        ret[i] = n;
    }

    ret
}

fn main() {
    let nums = vec![2, 1, 3, 5, 6];
    let ret = get_final_state(nums, 5, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 3, 5, 6];
        let expected = vec![8, 4, 6, 5, 6];
        let ret = get_final_state(nums, 5, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2];
        let expected = vec![16, 8];
        let ret = get_final_state(nums, 3, 4);
        assert_eq!(ret, expected);
    }
}
