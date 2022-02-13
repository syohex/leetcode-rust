fn inner_subsets(i: usize, nums: &Vec<i32>, acc: &Vec<i32>, ret: &mut Vec<Vec<i32>>) {
    if i >= nums.len() {
        ret.push(acc.clone());
        return;
    }

    inner_subsets(i + 1, nums, acc, ret);

    let mut tmp = acc.clone();
    tmp.push(nums[i]);
    inner_subsets(i + 1, nums, &mut tmp, ret);
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut acc = vec![];
    inner_subsets(0, &nums, &mut acc, &mut ret);
    ret
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = subsets(nums);
    println!("ret={:?}", ret);
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(got: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        use std::collections::HashSet;
        let gs: HashSet<Vec<i32>> = got.into_iter().collect();
        let es: HashSet<Vec<i32>> = expected.into_iter().collect();
        assert_eq!(gs, es);
    }

    #[test]
    fn test_subsets() {
        {
            let nums = vec![1, 2, 3];
            let expected = vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ];
            let got = subsets(nums);
            check(got, expected);
        }
        {
            let nums = vec![0];
            let expected = vec![vec![], vec![0]];
            let got = subsets(nums);
            check(got, expected);
        }
    }
}
