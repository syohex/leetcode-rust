fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut graph = vec![vec![]; n];
    let mut can_reaches = vec![0; n];

    for inv in invocations {
        graph[inv[0] as usize].push(inv[1] as usize);
        can_reaches[inv[1] as usize] += 1;
    }

    let mut suspicious = vec![false; n];
    suspicious[k] = true;

    let mut q = vec![k];
    while let Some(node) = q.pop() {
        for &next in &graph[node] {
            can_reaches[next] -= 1;
            if !suspicious[next] {
                suspicious[next] = true;
                q.push(next);
            }
        }
    }

    if (0..n)
        .filter(|&i| suspicious[i])
        .any(|i| can_reaches[i] >= 1)
    {
        (0..n as i32).collect()
    } else {
        let mut ret = vec![];
        for i in 0..n {
            if !suspicious[i] || can_reaches[i] >= 1 {
                ret.push(i as i32);
            }
        }

        ret
    }
}

fn main() {
    let ret = remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        remaining_methods(3, 2, vec![vec![1, 0], vec![2, 0]]),
        vec![0, 1, 2]
    );
    assert_eq!(
        remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]),
        vec![3, 4]
    );
    assert_eq!(
        remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]),
        vec![]
    );
}
