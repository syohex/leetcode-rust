fn find_lucky(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let h = arr.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    h.into_iter().fold(
        -1,
        |acc, (k, v)| {
            if k == v {
                std::cmp::max(acc, k)
            } else {
                acc
            }
        },
    )
}

fn main() {
    let arr = vec![2, 2, 3, 4];
    let ret = find_lucky(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![2, 2, 3, 4];
        let ret = find_lucky(arr);
        assert_eq!(ret, 2);
    }
    {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let ret = find_lucky(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![2, 2, 2, 3, 3];
        let ret = find_lucky(arr);
        assert_eq!(ret, -1);
    }
}
