fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut mat: Vec<(usize, i32)> = mat
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v.iter().filter(|&e| *e == 1).count(), i as i32))
        .collect();
    mat.sort_unstable();

    mat.into_iter().take(k as usize).map(|(_, i)| i).collect()
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
    println!("ret={ret:?}");
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
