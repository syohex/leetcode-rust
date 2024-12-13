fn find_score(nums: Vec<i32>) -> i64 {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    let mut q = BinaryHeap::new();
    let len = nums.len();
    for (i, n) in nums.into_iter().enumerate() {
        q.push(Reverse((n, i)));
    }

    let mut checked = HashSet::new();
    let mut ret = 0i64;
    while checked.len() != len {
        if let Some(Reverse((n, i))) = q.pop() {
            if checked.contains(&i) {
                continue;
            }

            ret += n as i64;
            checked.insert(i);
            if i >= 1 {
                checked.insert(i - 1);
            }
            if i + 1 < len {
                checked.insert(i + 1);
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![2, 1, 3, 4, 5, 2];
    let ret = find_score(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 3, 4, 5, 2];
        let ret = find_score(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![2, 3, 5, 1, 3, 2];
        let ret = find_score(nums);
        assert_eq!(ret, 5);
    }
}
