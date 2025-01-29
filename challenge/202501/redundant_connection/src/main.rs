fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    fn is_connected(
        node1: i32,
        node2: i32,
        graph: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if node1 == node2 {
            return true;
        }

        visited[node1 as usize] = true;
        for &next in &graph[node1 as usize] {
            if !visited[next as usize] && is_connected(next, node2, graph, visited) {
                return true;
            }
        }

        false
    }

    let n = edges.len();
    let mut graph = vec![vec![]; n + 1];
    for edge in edges {
        let mut visited = vec![false; n + 1];
        if is_connected(edge[0], edge[1], &graph, &mut visited) {
            return edge;
        }

        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }

    unreachable!("never reach here")
}

fn main() {
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let ret = find_redundant_connection(edges);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let ret = find_redundant_connection(edges);
        assert_eq!(ret, vec![2, 3]);
    }
    {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        let ret = find_redundant_connection(edges);
        assert_eq!(ret, vec![1, 4]);
    }
}
