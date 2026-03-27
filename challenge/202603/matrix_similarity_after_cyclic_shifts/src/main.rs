fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let k = k as usize;
    let (rows, cols) = (mat.len(), mat[0].len());

    for i in 0..rows {
        let is_even = i % 2 == 0;
        for j in 0..cols {
            let idx = if is_even {
                (j + k) % cols
            } else {
                (j + cols - k % cols) % cols
            };
            if mat[i][idx] != mat[i][j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
    let ret = are_similar(mat, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ret = are_similar(mat, 4);
        assert!(!ret)
    }
    {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let ret = are_similar(mat, 2);
        assert!(ret)
    }
    {
        let mat = vec![vec![2, 2], vec![2, 2]];
        let ret = are_similar(mat, 3);
        assert!(ret)
    }
}
