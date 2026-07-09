fn path_existence_queries(
    n: i32,
    nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let n = n as usize;
    let mut groups = vec![0; n];

    for i in 1..n {
        if nums[i] - nums[i - 1] <= max_diff {
            groups[i] = groups[i - 1];
        } else {
            groups[i] = i;
        }
    }

    queries
        .into_iter()
        .map(|q| groups[q[0] as usize] == groups[q[1] as usize])
        .collect()
}

fn main() {
    let n = 4;
    let nums = vec![2, 5, 6, 8];
    let max_diff = 2;
    let queries = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
    let ret = path_existence_queries(n, nums, max_diff, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let n = 4;
        let nums = vec![1, 1, 1, 1];
        let max_diff = 0;
        let queries = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let expected = vec![true, true, true, true];
        let ret = path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ret, expected);
    }
    {
        let n = 2;
        let nums = vec![1, 3];
        let max_diff = 1;
        let queries = vec![vec![0, 0], vec![0, 1]];
        let expected = vec![true, false];
        let ret = path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ret, expected);
    }
    {
        let n = 4;
        let nums = vec![2, 5, 6, 8];
        let max_diff = 2;
        let queries = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let expected = vec![false, false, true, true];
        let ret = path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ret, expected);
    }
}
