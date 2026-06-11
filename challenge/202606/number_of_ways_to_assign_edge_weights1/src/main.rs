fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let len = edges.len() + 1;
    let mut graph = vec![vec![]; len];

    for e in edges {
        let (a, b) = (e[0] as usize - 1, e[1] as usize - 1);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; len];
    let mut q = VecDeque::new();
    q.push_back(0);
    visited[0] = true;
    let mut depth = -1;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            if let Some(node) = q.pop_front() {
                for &next in &graph[node] {
                    if !visited[next] {
                        visited[next] = true;
                        q.push_back(next);
                    }
                }
            }
        }

        if !q.is_empty() {
            depth += 1;
        }
    }

    let mut ret = 1i64;
    let modulo = 1_000_000_007i64;
    while depth != 0 {
        ret = (ret * 2i64) % modulo;
        depth -= 1;
    }

    ret as i32
}

fn main() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
    let ret = assign_edge_weights(edges);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let edges = vec![vec![1, 2]];
        let ret = assign_edge_weights(edges);
        assert_eq!(ret, 1);
    }
    {
        let edges = vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]];
        let ret = assign_edge_weights(edges);
        assert_eq!(ret, 2);
    }
}
