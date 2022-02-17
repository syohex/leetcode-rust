fn combination_sum1(
    sum: i32,
    index: usize,
    candidates: &Vec<i32>,
    target: i32,
    acc: &mut Vec<i32>,
    ret: &mut Vec<Vec<i32>>,
) {
    if acc.iter().sum::<i32>() == target {
        ret.push(acc.clone());
        return;
    }

    if index >= candidates.len() {
        return;
    }

    for (i, cand) in candidates.iter().enumerate().skip(index) {
        if sum + cand <= target {
            acc.push(*cand);
            combination_sum1(sum + cand, i, candidates, target, acc, ret);
            acc.pop();
        }
    }
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut acc = vec![];
    let mut candidates = candidates;
    candidates.sort_unstable();

    combination_sum1(0, 0, &candidates, target, &mut acc, &mut ret);
    ret
}

fn main() {
    let candidates = vec![2, 3, 5];
    let ret = combination_sum(candidates, 8);
    println!("ret={:?}", ret);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn check(got: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        assert_eq!(got.len(), expected.len());

        let gs: HashSet<Vec<i32>> = HashSet::from_iter(got);
        let es: HashSet<Vec<i32>> = HashSet::from_iter(expected);

        assert_eq!(gs, es);
    }

    #[test]
    fn test_combination_sum() {
        {
            let candidates = vec![2, 3, 6, 7];
            let expected = vec![vec![2, 2, 3], vec![7]];
            let ret = combination_sum(candidates, 7);
            check(ret, expected);
        }
        {
            let candidates = vec![2, 3, 5];
            let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
            let ret = combination_sum(candidates, 8);
            check(ret, expected);
        }
        {
            let candidates = vec![2];
            let expected = vec![];
            let ret = combination_sum(candidates, 1);
            check(ret, expected);
        }
    }
}
