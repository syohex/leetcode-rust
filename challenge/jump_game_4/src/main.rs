fn min_jumps(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, n) in arr.iter().enumerate() {
        m.entry(*n).or_insert(vec![]).push(i);
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    let mut dp = vec![std::i32::MAX; arr.len()];
    dp[0] = 0;

    let end = arr.len() - 1;
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current == end {
            return dp[end];
        }

        if current >= 1 {
            if dp[current] + 1 < dp[current - 1] {
                dp[current - 1] = dp[current] + 1;
                q.push_back(current - 1);
            }
        }
        if current < end {
            if dp[current] + 1 < dp[current + 1] {
                dp[current + 1] = dp[current] + 1;
                q.push_back(current + 1);
            }
        }
        if let Some(candidates) = m.get(&arr[current]) {
            for cand in candidates {
                if dp[current] + 1 < dp[*cand] {
                    dp[*cand] = dp[current] + 1;
                    q.push_back(*cand);
                }
            }
            m.remove(&arr[current]);
        }
    }

    dp[end]
}

fn main() {
    let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
    let ret = min_jumps(arr);
    println!("ret={ret}");
}

#[test]
fn test_min_jumps() {
    {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(min_jumps(arr), 3);
    }
    {
        let arr = vec![7];
        assert_eq!(min_jumps(arr), 0);
    }
    {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(min_jumps(arr), 1);
    }
    {
        let arr = vec![7, 7, 2, 1, 7, 7, 7, 3, 4, 1];
        assert_eq!(min_jumps(arr), 3);
    }
    {
        let arr = vec![
            68, -94, -44, -18, -1, 18, -87, 29, -6, -87, -27, 37, -57, 7, 18, 68, -59, 29, 7, 53,
            -27, -59, 18, -1, 18, -18, -59, -1, -18, -84, -20, 7, 7, -87, -18, -84, -20, -27,
        ];
        assert_eq!(min_jumps(arr), 5);
    }
}
