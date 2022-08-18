fn min_set_size(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let half = (arr.len() / 2) as i32;
    let h = arr.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let mut v: Vec<i32> = h.into_iter().map(|(_, count)| count).collect();
    v.sort_unstable_by(|a, b| b.cmp(&a));

    let len = v.len();
    let mut sum = 0;
    for (i, count) in v.into_iter().enumerate() {
        sum += count;
        if sum >= half {
            return (i + 1) as i32;
        }
    }

    len as i32
}

fn main() {
    let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
    let ret = min_set_size(arr);
    println!("ret={ret}");
}

#[test]
fn test_min_set_size() {
    {
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        let ret = min_set_size(arr);
        assert_eq!(ret, 2);
    }
    {
        let arr = vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7];
        let ret = min_set_size(arr);
        assert_eq!(ret, 1);
    }
}
