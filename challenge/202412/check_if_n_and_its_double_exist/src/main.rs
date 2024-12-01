fn check_if_exist(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let h: HashMap<i32, Vec<usize>> =
        arr.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, n)| {
                acc.entry(*n).or_default().push(i);
                acc
            });

    for (i, n) in arr.into_iter().enumerate() {
        if let Some(v) = h.get(&(2 * n)) {
            for j in v {
                if i != *j {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let arr = vec![10, 2, 5, 3];
    let ret = check_if_exist(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![10, 2, 5, 3];
        assert!(check_if_exist(arr));
    }
    {
        let arr = vec![3, 1, 7, 11];
        assert!(!check_if_exist(arr));
    }
}
