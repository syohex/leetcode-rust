fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, a) in arr2.into_iter().enumerate() {
        h.insert(a, i);
    }

    let mut arr1 = arr1;
    arr1.sort_by(|a, b| match (h.get(a), h.get(b)) {
        (Some(i), Some(j)) => i.cmp(j),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(b),
    });

    arr1
}
fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    let ret = relative_sort_array(arr1, arr2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
        let arr2 = vec![2, 1, 4, 3, 9, 6];
        let expected = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];
        let ret = relative_sort_array(arr1, arr2);
        assert_eq!(ret, expected);
    }
    {
        let arr1 = vec![28, 6, 22, 8, 44, 17];
        let arr2 = vec![22, 28, 8, 6];
        let expected = vec![22, 28, 8, 6, 17, 44];
        let ret = relative_sort_array(arr1, arr2);
        assert_eq!(ret, expected);
    }
}
