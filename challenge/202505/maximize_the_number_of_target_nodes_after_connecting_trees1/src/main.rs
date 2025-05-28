fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    fn to_graph(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let nodes = edges.len() + 1;
        let mut graph = vec![vec![]; nodes];

        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        graph
    }

    fn count_nodes(node: usize, prev: usize, k: i32, graph: &[Vec<usize>]) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut ret = 1;
        for &next in &graph[node] {
            if next == prev {
                continue;
            }

            ret += count_nodes(next, node, k - 1, graph);
        }
        ret
    }

    let n = edges1.len() + 1;
    let m = edges2.len() + 1;

    let graph1 = to_graph(&edges1);
    let graph2 = to_graph(&edges2);

    let counts1: Vec<_> = (0..n)
        .map(|node| count_nodes(node, usize::MAX, k, &graph1))
        .collect();
    let counts2: Vec<_> = (0..m)
        .map(|node| count_nodes(node, usize::MAX, k - 1, &graph2))
        .collect();
    let max2 = counts2.into_iter().max().unwrap();

    counts1.into_iter().map(|n| n + max2).collect()
}

fn main() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    let edges2 = vec![
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![2, 7],
        vec![1, 4],
        vec![4, 5],
        vec![4, 6],
    ];
    let ret = max_target_nodes(edges1, edges2, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 7],
            vec![1, 4],
            vec![4, 5],
            vec![4, 6],
        ];
        let ret = max_target_nodes(edges1, edges2, 2);
        let expected = vec![9, 7, 9, 8, 8];
        assert_eq!(ret, expected);
    }
    {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let ret = max_target_nodes(edges1, edges2, 1);
        let expected = vec![6, 3, 3, 3, 3];
        assert_eq!(ret, expected);
    }
}
