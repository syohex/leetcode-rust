fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut q: BinaryHeap<_> = nums.into_iter().map(|n| Reverse(n as i64)).collect();
    let mut steps = 0;
    let k = k as i64;

    while q.len() >= 2 {
        if let Some(&Reverse(v)) = q.peek() {
            if v >= k {
                return steps;
            }
        }

        let x = q.pop().unwrap().0;
        let y = q.pop().unwrap().0;

        let v = std::cmp::min(x, y) * 2 + std::cmp::max(x, y);
        q.push(Reverse(v));

        steps += 1;
    }

    steps
}

fn main() {
    let nums = vec![2, 11, 10, 1, 3];
    let k = 10;
    let ret = min_operations(nums, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 11, 10, 1, 3];
        let k = 10;
        let ret = min_operations(nums, k);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 1, 2, 4, 9];
        let k = 20;
        let ret = min_operations(nums, k);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![
            1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999,
        ];
        let k = 1000000000;
        let ret = min_operations(nums, k);
        assert_eq!(ret, 2);
    }
}
