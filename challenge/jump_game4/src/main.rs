fn min_jumps(arr: Vec<i32>) -> i32 {
    use std::collections::{HashMap, VecDeque};

    let len = arr.len();
    if len == 1 {
        return 0;
    }

    let mut graph = HashMap::new();
    for (i, n) in arr.iter().enumerate() {
        graph.entry(n).or_insert(vec![]).push(i);
    }

    let mut q = VecDeque::new();
    q.push_back(0);

    let mut steps = 0;
    while !q.is_empty() {
        let q_len = q.len();
        for _ in 0..q_len {
            let node = q.pop_front().unwrap();
            if node == len - 1 {
                return steps;
            }

            if let Some(nexts) = graph.remove(&arr[node]) {
                for next in nexts {
                    q.push_back(next);
                }
            }

            if node >= 1 {
                q.push_back(node - 1);
            }
            if node + 1 < len {
                q.push_back(node + 1);
            }
        }

        steps += 1;
    }

    unreachable!("never reach here");
}

fn main() {
    let ret = min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    assert_eq!(min_jumps(vec![7]), 0);
    assert_eq!(min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
}
