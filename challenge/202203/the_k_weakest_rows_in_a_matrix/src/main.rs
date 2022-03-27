fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut v: Vec<(usize, i32)> = mat
        .into_iter()
        .map(|v| v.into_iter().fold(0, |acc, n| acc + n))
        .enumerate()
        .collect();

    v.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    v.into_iter().take(k as usize).map(|a| a.0 as i32).collect()
}

fn main() {
    let mat = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];
    let ret = k_weakest_rows(mat, 3);
    println!("ret={:?}", ret);
}

#[test]
fn test_k_weakest_rows() {
    {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let expected = vec![2, 0, 3];
        let ret = k_weakest_rows(mat, 3);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let expected = vec![0, 2];
        let ret = k_weakest_rows(mat, 2);
        assert_eq!(ret, expected);
    }
}
