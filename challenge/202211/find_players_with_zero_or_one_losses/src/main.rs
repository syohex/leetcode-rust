fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    let mut winners: HashMap<i32, i32> = HashMap::new();
    let mut losers: HashMap<i32, i32> = HashMap::new();

    for m in matches {
        *winners.entry(m[0]).or_insert(0) += 1;
        *losers.entry(m[1]).or_insert(0) += 1;
    }

    let mut all_winners = vec![];
    let mut one_losers = vec![];
    for (k, _) in winners.iter() {
        if !losers.contains_key(k) {
            all_winners.push(*k);
        }
    }

    for (k, v) in losers.iter() {
        if *v == 1 {
            one_losers.push(*k);
        }
    }

    all_winners.sort_unstable();
    one_losers.sort_unstable();

    vec![all_winners, one_losers]
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
