fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph = vec![vec![]; n];

    for e in edges {
        graph[e[0] as usize].push(e[1] as usize);
        graph[e[1] as usize].push(e[0] as usize);
    }

    let mut ret = 0;
    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        let mut q = vec![i];
        let mut nodes = vec![];
        while let Some(node) = q.pop() {
            nodes.push(node);

            for &next in &graph[node] {
                if !visited[next] {
                    visited[next] = true;
                    q.push(next);
                }
            }
        }

        let node_count = nodes.len();
        if nodes.iter().all(|node| graph[*node].len() + 1 == node_count) {
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
