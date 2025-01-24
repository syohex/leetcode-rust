fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    fn f(i: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> bool {
        if graph[i].is_empty() {
            return true;
        }
        if visited[i] {
            return false;
        }

        visited[i] = true;
        for &next in &graph[i] {
            let next = next as usize;
            if !f(next, graph, visited) {
                return false;
            }
        }
        visited[i] = false;

        return true;
    }

    let n = graph.len();
    let mut ret = vec![];
    let mut visited = vec![false; n];
    for i in 0..n {
        if f(i, &graph, &mut visited) {
            ret.push(i as i32);
        }
    }

    ret
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![2, 3],
        vec![5],
        vec![0],
        vec![5],
        vec![],
        vec![],
    ];
    let ret = eventual_safe_nodes(graph);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let expected = vec![2, 4, 5, 6];
        let ret = eventual_safe_nodes(graph);
        assert_eq!(ret, expected);
    }
    {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let expected = vec![4];
        let ret = eventual_safe_nodes(graph);
        assert_eq!(ret, expected);
    }
    {
        let graph = vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![]];
        let expected = vec![0, 1, 2, 3, 4];
        let ret = eventual_safe_nodes(graph);
        assert_eq!(ret, expected);
    }
}
