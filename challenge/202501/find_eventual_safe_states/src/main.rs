fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    fn f(
        i: usize,
        graph: &Vec<Vec<i32>>,
        checked: &mut Vec<bool>,
        in_path: &mut Vec<bool>,
    ) -> bool {
        if in_path[i] {
            return true;
        }
        if checked[i] {
            return false;
        }

        checked[i] = true;
        in_path[i] = true;
        for &next in &graph[i] {
            let next = next as usize;
            if f(next, graph, checked, in_path) {
                return true;
            }
        }
        in_path[i] = false;

        false
    }

    let n = graph.len();
    let mut in_path = vec![false; n];
    let mut checked = vec![false; n];
    for i in 0..n {
        f(i, &graph, &mut checked, &mut in_path);
    }

    let mut ret = vec![];
    for i in 0..n {
        if !in_path[i] {
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
