fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    use std::collections::{HashMap, VecDeque};

    let mut sorted = nums.clone();
    sorted.sort_unstable();

    let mut group_map = HashMap::new();
    let mut group_id = 0;
    let mut groups = HashMap::new();

    group_map.insert(sorted[0], group_id);
    groups.entry(group_id).or_insert(VecDeque::new()).push_back(sorted[0]);

    for (i, &n) in sorted.iter().enumerate().skip(1) {
        if n - sorted[i - 1] > limit {
            group_id += 1;
        }

        group_map.insert(n, group_id);
        groups
            .entry(group_id)
            .or_insert(VecDeque::new())
            .push_back(n);
    }

    let mut ret = vec![];
    for n in nums.into_iter() {
        if let Some(id) = group_map.get(&n) {
            if let Some(v) = groups.get_mut(id) {
                if let Some(m) = v.pop_front() {
                    ret.push(m);
                }
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 7, 6, 18, 2, 1];
    let ret = lexicographically_smallest_array(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 5, 3, 9, 8];
        let expected = vec![1, 3, 5, 8, 9];
        let ret = lexicographically_smallest_array(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 7, 6, 18, 2, 1];
        let expected = vec![1, 6, 7, 18, 1, 2];
        let ret = lexicographically_smallest_array(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 7, 28, 19, 10];
        let expected = vec![1, 7, 28, 19, 10];
        let ret = lexicographically_smallest_array(nums, 3);
        assert_eq!(ret, expected);
    }
}
