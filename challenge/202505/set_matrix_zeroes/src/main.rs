fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    use std::collections::HashSet;
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut zero_rows = HashSet::new();
    let mut zero_cols = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                zero_rows.insert(i);
                zero_cols.insert(j);
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if zero_rows.contains(&i) || zero_cols.contains(&j) {
                matrix[i][j] = 0;
            }
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut matrix);
    println!("ret={matrix:?}");
}

#[test]
fn test() {
    {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);

        let expeccted = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(matrix, expeccted);
    }
    {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);

        let expeccted = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        assert_eq!(matrix, expeccted);
    }
}
