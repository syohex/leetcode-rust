fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = n as usize;
    let mut graph = vec![vec![]; n];

    for edge in edges {
        graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        graph[edge[1] as usize].push((edge[0] as usize, edge[2] * 2));
    }

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0usize));

    let mut costs = vec![i32::MAX; n];
    while let Some((Reverse(cost), node)) = q.pop() {
        for (next_node, next_cost) in &graph[node] {
            let v = cost + *next_cost;
            if v < costs[*next_node] {
                costs[*next_node] = v;
                q.push((Reverse(v), *next_node));
            }
        }
    }

    if costs[n - 1] == i32::MAX {
        -1
    } else {
        costs[n - 1]
    }
}

fn main() {
    let edges = vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]];
    let ret = min_cost(4, edges);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let edges = vec![vec![2, 3, 25], vec![2, 1, 18], vec![3, 1, 2]];
        let ret = min_cost(4, edges);
        assert_eq!(ret, -1);
    }
    {
        let edges = vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]];
        let ret = min_cost(4, edges);
        assert_eq!(ret, 5);
    }
    {
        let edges = vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]];
        let ret = min_cost(4, edges);
        assert_eq!(ret, 3);
    }
}
