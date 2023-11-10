fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut count: HashMap<i32, i32> = HashMap::new();
    for p in &adjacent_pairs {
        if let Some(v) = graph.get_mut(&p[0]) {
            v.push(p[1]);
        } else {
            graph.insert(p[0], vec![p[1]]);
        }
        if let Some(v) = graph.get_mut(&p[1]) {
            v.push(p[0]);
        } else {
            graph.insert(p[1], vec![p[0]]);
        }

        *count.entry(p[0]).or_insert(0) += 1;
        *count.entry(p[1]).or_insert(0) += 1;
    }

    let &parent = count.iter().find(|(_, &v)| v == 1).unwrap().0;

    let mut ret = vec![];
    let mut checked = HashSet::new();

    let mut next = parent;
    while checked.len() < count.len() {
        ret.push(next);
        checked.insert(next);

        if let Some(nodes) = graph.get(&next) {
            for node in nodes {
                if checked.contains(node) {
                    continue;
                }

                next = *node;
                break;
            }
        }
    }

    ret
}

fn main() {
    let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
    let ret = restore_array(adjacent_pairs);
    println!("ret={ret:?}");
}

#[test]
fn test_restore_array() {
    {
        let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
        let expected1 = vec![1, 2, 3, 4];
        let expected2 = vec![4, 3, 2, 1];
        let ret = restore_array(adjacent_pairs);
        assert!(ret == expected1 || ret == expected2);
    }
    {
        let adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
        let expected1 = vec![-2, 4, 1, -3];
        let expected2 = vec![-3, 1, 4, -2];
        let ret = restore_array(adjacent_pairs);
        assert!(ret == expected1 || ret == expected2);
    }
    {
        let adjacent_pairs = vec![vec![1000, -1000]];
        let expected1 = vec![1000, -1000];
        let expected2 = vec![-1000, 1000];
        let ret = restore_array(adjacent_pairs);
        assert!(ret == expected1 || ret == expected2);
    }
}
