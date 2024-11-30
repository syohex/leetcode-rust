fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    fn f(current: i32, graph: &mut HashMap<i32, VecDeque<i32>>, acc: &mut Vec<Vec<i32>>) {
        while let Some(prev) = graph.get_mut(&current).unwrap().pop_front() {
            f(prev, graph, acc);
            acc.push(vec![prev, current]);
        }
    }

    let mut graph = HashMap::new();
    let mut in_outs = HashMap::new();
    for p in &pairs {
        graph.entry(p[1]).or_insert(VecDeque::new()).push_back(p[0]);
        graph.entry(p[0]).or_insert(VecDeque::new());
        *in_outs.entry(p[1]).or_insert(0) += 1;
        *in_outs.entry(p[0]).or_insert(0) -= 1;
    }

    let mut start = -1;
    for (node, count) in &in_outs {
        if *count == 1 {
            start = *node;
            break;
        }
    }
    if start == -1 {
        start = pairs[0][0];
    }
    let mut acc = vec![];
    f(start, &mut graph, &mut acc);

    acc
}

fn main() {
    let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
    let ret = valid_arrangement(pairs);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
        let expected = vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]];
        let ret = valid_arrangement(pairs);
        assert_eq!(ret, expected);
    }
    {
        let pairs = vec![vec![1, 3], vec![3, 2], vec![2, 1]];
        let expected = vec![vec![1, 3], vec![3, 2], vec![2, 1]];
        let ret = valid_arrangement(pairs);
        assert_eq!(ret, expected);
    }
    {
        let pairs = vec![vec![1, 2], vec![1, 3], vec![2, 1]];
        let expected = vec![vec![1, 2], vec![2, 1], vec![1, 3]];
        let ret = valid_arrangement(pairs);
        assert_eq!(ret, expected);
    }
    {
        let pairs = vec![
            vec![8, 5],
            vec![8, 7],
            vec![0, 8],
            vec![0, 5],
            vec![7, 0],
            vec![5, 0],
            vec![0, 7],
            vec![8, 0],
            vec![7, 8],
        ];
        let expected = vec![
            vec![8, 0],
            vec![0, 5],
            vec![5, 0],
            vec![0, 7],
            vec![7, 8],
            vec![8, 7],
            vec![7, 0],
            vec![0, 8],
            vec![8, 5],
        ];
        let ret = valid_arrangement(pairs);
        assert_eq!(ret, expected);
    }
}
