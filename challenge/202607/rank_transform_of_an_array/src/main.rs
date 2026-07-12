fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    if arr.is_empty() {
        return vec![];
    }

    let mut v = arr.clone();
    v.sort_unstable();

    let mut h = HashMap::new();
    h.insert(v[0], 1);
    let mut rank = 1;
    for i in 1..v.len() {
        if v[i] != v[i - 1] {
            rank += 1;
        }
        h.insert(v[i], rank);
    }

    arr.into_iter().map(|n| *h.get(&n).unwrap()).collect()
}

fn main() {
    let ret = array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(array_rank_transform(vec![]), []);
    assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), [4, 1, 2, 3]);
    assert_eq!(array_rank_transform(vec![100, 100, 100]), [1, 1, 1]);
    assert_eq!(
        array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        [5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}
