fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    use std::collections::{HashMap, VecDeque};

    let mut v = nums.clone();
    v.sort_unstable();

    let mut groups = HashMap::new();
    let mut group_nums = HashMap::new();
    let mut id = -1;
    let mut prev = -100;

    for n in v {
        if n - prev > limit {
            id += 1;
        }

        groups.insert(n, id);
        group_nums.entry(id).or_insert(VecDeque::new()).push_back(n);
        prev = n;
    }

    let mut ret = vec![];
    for n in nums {
        if let Some(id) = groups.get(&n)
            && let Some(q) = group_nums.get_mut(id)
            && let Some(m) = q.pop_front()
        {
            ret.push(m);
        }
    }

    ret
}

fn main() {
    let ret = lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
        vec![1, 3, 5, 8, 9]
    );
    assert_eq!(
        lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
        vec![1, 6, 7, 18, 1, 2]
    );
    assert_eq!(
        lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
        vec![1, 7, 28, 19, 10]
    );
}
