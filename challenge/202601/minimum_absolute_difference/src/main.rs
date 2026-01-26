fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    let mut arr = arr;
    arr.sort_unstable();

    let mut h = HashMap::new();
    let mut min_diff = i32::MAX;
    for i in 1..arr.len() {
        let diff = arr[i] - arr[i - 1];
        h.entry(diff)
            .or_insert(vec![])
            .push(vec![arr[i - 1], arr[i]]);
        min_diff = std::cmp::min(min_diff, diff);
    }

    if let Some(v) = h.remove(&min_diff) {
        v
    } else {
        unreachable!("never reach here");
    }
}

fn main() {
    let arr = vec![4, 2, 1, 3];
    let ret = minimum_abs_difference(arr);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let arr = vec![4, 2, 1, 3];
        let ret = minimum_abs_difference(arr);
        assert_eq!(ret, [[1, 2], [2, 3], [3, 4]]);
    }
    {
        let arr = vec![1, 3, 6, 10, 15];
        let ret = minimum_abs_difference(arr);
        assert_eq!(ret, [[1, 3]]);
    }
    {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let ret = minimum_abs_difference(arr);
        assert_eq!(ret, [[-14, -10], [19, 23], [23, 27]]);
    }
}
