fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::{HashMap, VecDeque};

    let len = nums.len() as i32;
    let mut h = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        h.entry(*n).or_insert(VecDeque::new()).push_back(i as i32);
    }

    for v in h.values_mut() {
        let first = *v.front().unwrap();
        let last = *v.iter().last().unwrap();
        v.push_front(last - len);
        v.push_back(first + len);
    }

    let mut ret = vec![];
    for q in queries {
        let target = nums[q as usize];
        if let Some(v) = h.get(&target) {
            if v.len() <= 3 {
                ret.push(-1);
            } else {
                let pos = v.partition_point(|p| *p < q);
                dbg!(pos, q, target, &v);
                let min_pos = std::cmp::min(v[pos + 1] - v[pos], v[pos] - v[pos - 1]);
                ret.push(min_pos);
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 1, 4, 1, 3, 2];
    let queries = vec![0, 3, 5];
    let ret = solve_queries(nums, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![12, 19, 12, 8, 12, 10];
        let queries = vec![0, 5, 3, 1];
        let ret = solve_queries(nums, queries);
        assert_eq!(ret, [2, -1, -1, -1]);
    }
    {
        let nums = vec![1, 3, 1, 4, 1, 3, 2];
        let queries = vec![0, 3, 5];
        let ret = solve_queries(nums, queries);
        assert_eq!(ret, [2, -1, 3]);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![0, 1, 2, 3];
        let ret = solve_queries(nums, queries);
        assert_eq!(ret, [-1, -1, -1, -1]);
    }
}
