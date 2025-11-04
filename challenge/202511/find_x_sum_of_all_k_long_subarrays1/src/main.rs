fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut ret = vec![];
    for v in nums.windows(k as usize) {
        let mut h = HashMap::new();
        for n in v.iter() {
            *h.entry(*n).or_insert(0) += 1;
        }

        let mut freq: Vec<_> = h.into_iter().collect();
        freq.sort_unstable_by(
            |(k1, v1), (k2, v2)| {
                if v1 == v2 { k2.cmp(k1) } else { v2.cmp(v1) }
            },
        );

        ret.push(freq.into_iter().take(x as usize).map(|(k, v)| k * v).sum());
    }

    ret
}
fn main() {
    let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
    let ret = find_x_sum(nums, 6, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let ret = find_x_sum(nums, 6, 2);
        assert_eq!(ret, [6, 10, 12])
    }
    {
        let nums = vec![3, 8, 7, 8, 7, 5];
        let ret = find_x_sum(nums, 2, 2);
        assert_eq!(ret, [11, 15, 15, 15, 12])
    }
}
