fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    fn f(len: usize, h: &mut HashMap<i32, i32>, acc: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if acc.len() == len {
            ret.push(acc.clone());
            return;
        }

        for (k, v) in h.iter() {
            if *v != 0 {
                acc.push(*k);

                let mut h2 = h.clone();
                h2.insert(*k, *v - 1);
                f(len, &mut h2, acc, ret);
                acc.pop();
            }
        }
    }

    let mut h = HashMap::new();
    for n in nums.iter() {
        *h.entry(*n).or_insert(0) += 1;
    }

    let mut acc = vec![];
    let mut ret = vec![];

    f(nums.len(), &mut h, &mut acc, &mut ret);

    ret
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = permute_unique(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_permute_unique() {
    fn check(got: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let a: HashSet<Vec<i32>> = got.into_iter().collect();
        let b: HashSet<Vec<i32>> = expected.into_iter().collect();

        assert_eq!(a, b);
    }

    {
        let nums = vec![1, 1, 2];
        let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        let ret = permute_unique(nums);
        check(ret, expected);
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
        let ret = permute_unique(nums);
        check(ret, expected);
    }
}
