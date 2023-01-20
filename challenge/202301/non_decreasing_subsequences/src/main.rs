fn find_sequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn f(i: usize, nums: &Vec<i32>, acc: &mut Vec<i32>, ret: &mut HashSet<Vec<i32>>) {
        if i == nums.len() {
            if acc.len() >= 2 {
                ret.insert(acc.clone());
            }
            return;
        }

        f(i + 1, nums, acc, ret);

        if acc.is_empty() {
            acc.push(nums[i]);
            f(i + 1, nums, acc, ret);
            acc.pop();
        } else {
            if *acc.last().unwrap() <= nums[i] {
                acc.push(nums[i]);
                f(i + 1, nums, acc, ret);
                acc.pop();
            }
        }
    }

    let mut acc = vec![];
    let mut ret = HashSet::new();
    f(0, &nums, &mut acc, &mut ret);

    ret.into_iter().collect()
}

fn main() {
    let nums = vec![4, 6, 7, 7];
    let ret = find_sequences(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_find_subsequences() {
    fn check(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let a: HashSet<Vec<i32>> = a.into_iter().collect();
        let b: HashSet<Vec<i32>> = b.into_iter().collect();
        assert_eq!(a, b);
    }

    {
        let nums = vec![4, 6, 7, 7];
        let expected = vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
        ];
        let ret = find_sequences(nums);
        check(ret, expected);
    }
    {
        let nums = vec![4, 4, 3, 2, 1];
        let expected = vec![vec![4, 4]];
        let ret = find_sequences(nums);
        check(ret, expected);
    }
}
