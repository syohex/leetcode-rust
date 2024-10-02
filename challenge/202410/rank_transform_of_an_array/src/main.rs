fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let s: HashSet<i32> = arr.iter().cloned().collect();
    let mut v: Vec<i32> = s.into_iter().collect();
    v.sort_unstable();

    let mut h = HashMap::new();
    for (i, n) in v.into_iter().enumerate() {
        h.insert(n, (i + 1) as i32);
    }

    arr.into_iter().map(|n| *h.get(&n).unwrap()).collect()
}

fn main() {
    let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
    let ret = array_rank_transform(arr);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let arr = vec![40, 10, 20, 30];
        let expected = vec![4, 1, 2, 3];
        let ret = array_rank_transform(arr);
        assert_eq!(ret, expected);
    }
    {
        let arr = vec![100, 100, 100];
        let expected = vec![1, 1, 1];
        let ret = array_rank_transform(arr);
        assert_eq!(ret, expected);
    }
    {
        let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        let expected = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];
        let ret = array_rank_transform(arr);
        assert_eq!(ret, expected);
    }
}
