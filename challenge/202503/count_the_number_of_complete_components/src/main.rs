fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
        graph[edge[1] as usize].push(edge[0] as usize);
    }

    let mut ret = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut q = VecDeque::new();
        let mut nodes = vec![];
        q.push_back(i);
        visited[i] = true;

        while let Some(node) = q.pop_front() {
            nodes.push(node);

            for &next in &graph[node] {
                if !visited[next] {
                    q.push_back(next);
                    visited[next] = true;
                }
            }
        }

        let nodes_len = nodes.len();
        if nodes.iter().all(|&node| graph[node].len() == nodes_len - 1) {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]];
    let ret = count_complete_components(n, edges);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]];
        let ret = count_complete_components(n, edges);
        assert_eq!(ret, 3);
    }
    {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]];
        let ret = count_complete_components(n, edges);
        assert_eq!(ret, 1);
    }
}
