fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;

    let sets: Vec<HashSet<i32>> = nums
        .iter()
        .map(|v| {
            let h: HashSet<i32> = v.iter().cloned().collect();
            h
        })
        .collect();

    let mut ret: Vec<i32> = sets
        .iter()
        .skip(1)
        .fold(sets[0].clone(), |acc, s| {
            acc.intersection(s).map(|i| *i).collect()
        })
        .into_iter()
        .collect();

    ret.sort_unstable();
    ret
}

fn main() {
    let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
    let ret = intersection(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_intersection() {
    {
        let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
        let expected = vec![3, 4];
        let ret = intersection(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let ret = intersection(nums);
        assert!(ret.is_empty());
    }
    {
        let nums = vec![vec![1, 1, 1], vec![1, 2, 2]];
        let expected = vec![1];
        let ret = intersection(nums);
        assert_eq!(ret, expected);
    }
}
