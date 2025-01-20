fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let (rows, cols) = (mat.len(), mat[0].len());
    let mut h = HashMap::new();
    for i in 0..rows {
        for j in 0..cols {
            h.insert(mat[i][j], (i, j));
        }
    }

    let mut row_sum = vec![0; rows];
    let mut col_sum = vec![0; cols];
    for (i, n) in arr.into_iter().enumerate() {
        if let Some(&(row, col)) = h.get(&n) {
            row_sum[row] += 1;
            col_sum[col] += 1;

            if row_sum[row] == cols || col_sum[col] == rows {
                return i as i32;
            }
        }
    }

    unreachable!("never reach here")
}
fn main() {
    let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
    let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
    let ret = first_complete_index(arr, mat);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 3, 4, 2];
        let mat = vec![vec![1, 4], vec![2, 3]];
        let ret = first_complete_index(arr, mat);
        assert_eq!(ret, 2);
    }
    {
        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        let ret = first_complete_index(arr, mat);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![1,4,5,2,6,3];
        let mat = vec![vec![4,3,5], vec![1,2,6]];
        let ret = first_complete_index(arr, mat);
        assert_eq!(ret, 1);
    }
}
