fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;

    let n = n as usize;
    let mut graph = vec![vec![]; n];

    for i in 0..(n - 1) {
        graph[i].push(i + 1);
    }

    let mut ret = vec![];
    for query in queries {
        graph[query[0] as usize].push(query[1] as usize);

        let mut q = VecDeque::new();
        q.push_back(0);

        let mut steps = 0;
        'outer: while !q.is_empty() {
            let len = q.len();

            for _ in 0..len {
                let node = q.pop_front().unwrap();
                if node == n - 1 {
                    break 'outer;
                }

                for next in &graph[node] {
                    q.push_back(*next);
                }
            }

            steps += 1;
        }

        ret.push(steps);
    }

    ret
}

fn main() {
    let n = 5;
    let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
    let ret = shortest_distance_after_queries(n, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let n = 5;
        let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
        let expected = vec![3, 2, 1];
        let ret = shortest_distance_after_queries(n, queries);
        assert_eq!(ret, expected);
    }
    {
        let n = 4;
        let queries = vec![vec![0, 3], vec![0, 2]];
        let expected = vec![1, 1];
        let ret = shortest_distance_after_queries(n, queries);
        assert_eq!(ret, expected);
    }
}
