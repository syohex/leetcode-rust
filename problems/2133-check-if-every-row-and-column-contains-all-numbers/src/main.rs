fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    let len = matrix.len();

    for i in 0..len {
        let mut check_row = vec![false; len];
        let mut check_col = vec![false; len];
        for j in 0..len {
            let index1 = (matrix[i][j] - 1) as usize;
            let index2 = (matrix[j][i] - 1) as usize;
            if check_row[index1] || check_col[index2] {
                return false;
            }

            check_row[index1] = true;
            check_col[index2] = true;
        }
    }

    true
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    println!("ret={}", check_valid(matrix));
}

#[test]
fn test_check_valid() {
    {
        let matrix = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
        assert!(check_valid(matrix));
    }
    {
        let matrix = vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]];
        assert!(!check_valid(matrix));
    }
}
