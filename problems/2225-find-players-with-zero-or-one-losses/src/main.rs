fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    let mut win = HashMap::new();
    let mut lose = HashMap::new();
    for v in &matches {
        *win.entry(v[0]).or_insert(0) += 1;
        *lose.entry(v[1]).or_insert(0) += 1;
    }

    let mut v1 = vec![];
    let mut v2 = vec![];

    for (k, _) in &win {
        if !lose.contains_key(k) {
            v1.push(*k);
        }
    }

    for (k, v) in &lose {
        if *v == 1 {
            v2.push(*k);
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    vec![v1, v2]
}

fn main() {
    let matches = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ];
    let ret = find_winners(matches);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_winners() {
    {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        let ret = find_winners(matches);
        assert_eq!(ret, expected);
    }
    {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let expected = vec![vec![1, 2, 5, 6], vec![]];
        let ret = find_winners(matches);
        assert_eq!(ret, expected);
    }
}
