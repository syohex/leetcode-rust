fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut ret = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            ret[j][i] = matrix[i][j];
        }
    }

    ret
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let ret = transpose(matrix);
    println!("ret={:?}", ret);
}

#[test]
fn test_transpose() {
    {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        let ret = transpose(matrix);
        assert_eq!(ret, expected);
    }
    {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let ret = transpose(matrix);
        assert_eq!(ret, expected);
    }
    {
        let matrix = vec![vec![1, 2, 3]];
        let expected = vec![vec![1], vec![2], vec![3]];
        let ret = transpose(matrix);
        assert_eq!(ret, expected);
    }
}
