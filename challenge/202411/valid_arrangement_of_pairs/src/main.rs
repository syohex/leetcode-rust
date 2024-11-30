fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    fn f(
        i: i32,
        limit: usize,
        graph: &HashMap<i32, Vec<(usize, i32, i32)>>,
        used: &mut Vec<bool>,
        acc: &mut Vec<Vec<i32>>,
    ) -> bool {
        if acc.len() == limit {
            return true;
        }

        if let Some(v) = graph.get(&i) {
            for &(j, start, end) in v {
                if !used[j] {
                    used[j] = true;
                    acc.push(vec![start, end]);
                    if f(end, limit, graph, used, acc) {
                        return true;
                    }
                    acc.pop();
                    used[j] = false;
                }
            }
        }

        false
    }

    let mut graph = HashMap::new();
    let mut ins = HashMap::new();
    for (i, p) in pairs.iter().enumerate() {
        graph.entry(p[0]).or_insert(vec![]).push((i, p[0], p[1]));
        *ins.entry(p[1]).or_insert(0) += 1;
    }

    let len = pairs.len();
    let mut ret = vec![];
    let mut used = vec![false; len];
    let mut start = -1;
    for (i, p) in pairs.iter().enumerate() {
        let in_ = *ins.get(&p[0]).unwrap_or(&0);
        if in_ == 0 {
            start = p[1];
            ret.push(p.clone());
            used[i] = true;
            break;
        }
    }
    if start == -1 {
        start = pairs[0][1];
        ret.push(pairs[0].clone());
        used[0] = true;
    }

    f(start, len, &graph, &mut used, &mut ret);
    ret
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
}
