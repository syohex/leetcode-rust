fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        let mut r = i + 1;
        let mut c = 1;

        while r < rows && c < cols {
            if matrix[i][0] != matrix[r][c] {
                return false;
            }

            r += 1;
            c += 1;
        }
    }

    for j in 0..cols {
        let mut r = 1;
        let mut c = j + 1;

        while r < rows && c < cols {
            if matrix[0][j] != matrix[r][c] {
                return false;
            }

            r += 1;
            c += 1;
        }
    }

    true
}
fn main() {
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
    let ret = is_toeplitz_matrix(matrix);
    println!("ret={ret}");
}

#[test]
fn test_is_toeplitz_matrix() {
    {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        let ret = is_toeplitz_matrix(matrix);
        assert!(ret);
    }
    {
        let matrix = vec![vec![1, 2], vec![2, 2]];
        let ret = is_toeplitz_matrix(matrix);
        assert!(!ret);
    }
    {
        let matrix = vec![vec![84]];
        let ret = is_toeplitz_matrix(matrix);
        assert!(ret);
    }
}
