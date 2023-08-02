fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn f(
        nums: &Vec<i32>,
        acc: &mut Vec<i32>,
        len: usize,
        used: &mut Vec<bool>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if len == nums.len() {
            ret.push(acc.clone());
            return;
        }

        for (i, num) in nums.iter().enumerate() {
            if used[i] {
                continue;
            }

            used[i] = true;
            acc[len] = *num;
            f(nums, acc, len + 1, used, ret);
            used[i] = false;
        }
    }

    let mut acc = vec![0; nums.len()];
    let mut used = vec![false; nums.len()];
    let mut ret = vec![];

    f(&nums, &mut acc, 0, &mut used, &mut ret);

    ret
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = permute(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_permute() {
    fn check(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let sa: HashSet<Vec<i32>> = a.into_iter().collect();
        let sb: HashSet<Vec<i32>> = b.into_iter().collect();
        assert_eq!(sa, sb);
    }

    {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let ret = permute(nums);
        check(ret, expected);
    }
    {
        let nums = vec![0, 1];
        let expected = vec![vec![0, 1], vec![1, 0]];
        let ret = permute(nums);
        check(ret, expected);
    }
    {
        let nums = vec![1];
        let expected = vec![vec![1]];
        let ret = permute(nums);
        check(ret, expected);
    }
}
