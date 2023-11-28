fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut m = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let col = (j + k as usize) % cols;
            m[i][col] = mat[i][j];
        }
    }

    mat == m
}

fn main() {
    let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
    let ret = are_similar(mat, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let ret = are_similar(mat, 2);
        assert!(ret);
    }
    {
        let mat = vec![vec![2, 2], vec![2, 2]];
        let ret = are_similar(mat, 3);
        assert!(ret);
    }
    {
        let mat = vec![vec![1, 2]];
        let ret = are_similar(mat, 1);
        assert!(!ret);
    }
}
