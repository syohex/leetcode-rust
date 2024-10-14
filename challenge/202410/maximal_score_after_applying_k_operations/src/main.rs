fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;

    let mut ret = 0i64;
    let mut h: BinaryHeap<_> = nums.into_iter().map(|n| n as i64).collect();

    for _ in 0..k {
        let n = h.pop().unwrap();
        ret += n;

        h.push((n as f64 / 3.0).ceil() as i64);
    }

    ret
}

fn main() {
    let nums = vec![1, 10, 3, 3, 3];
    let ret = max_kelements(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 10, 10, 10, 10];
        let ret = max_kelements(nums, 5);
        assert_eq!(ret, 50);
    }
    {
        let nums = vec![1, 10, 3, 3, 3];
        let ret = max_kelements(nums, 3);
        assert_eq!(ret, 17);
    }
}
