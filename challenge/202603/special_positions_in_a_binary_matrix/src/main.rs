fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (mat.len(), mat[0].len());
    let mut ret = 0;
    for i in 0..rows {
        let mut valid_col = None;
        for j in 0..cols {
            if mat[i][j] == 1 {
                if valid_col.is_none() {
                    valid_col = Some(j);
                } else {
                    valid_col = None;
                    break;
                }
            }
        }

        if let Some(col) = valid_col {
            let mut col_ones = 0;
            for k in 0..rows {
                col_ones += mat[k][col];
                if col_ones >= 2 {
                    break;
                }
            }
            if col_ones == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let ret = num_special(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let ret = num_special(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        let ret = num_special(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let ret = num_special(grid);
        assert_eq!(ret, 3);
    }
}
