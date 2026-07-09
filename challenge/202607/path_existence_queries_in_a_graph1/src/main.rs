fn path_existence_queries(
    n: i32,
    nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    use std::collections::VecDeque;

    let n = n as usize;
    let mut graph = vec![vec![]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            if nums[j] - nums[i] > max_diff {
                break;
            }

            graph[i].push(j);
            graph[j].push(i);
        }
    }

    let mut group = vec![n + 1; n];
    for i in 0..n {
        if group[i] != n + 1 {
            continue;
        }

        group[i] = i;
        let mut q = VecDeque::new();
        q.push_back(i);

        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let node = q.pop_front().unwrap();
                for &next in &graph[node] {
                    if group[next] == n + 1 {
                        group[next] = i;
                        q.push_back(next)
                    }
                }
            }
        }
    }

    let mut ret = vec![];
    for q in queries {
        ret.push(group[q[0] as usize] == group[q[1] as usize]);
    }

    ret
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
