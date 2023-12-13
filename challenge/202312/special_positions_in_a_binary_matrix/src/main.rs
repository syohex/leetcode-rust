fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let rows = mat.len();
    let cols = mat[0].len();

    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 1 {
                let mut row_ones = 0;
                for x in 0..rows {
                    if mat[x][j] == 1 {
                        row_ones += 1;
                    }
                }

                let mut col_ones = 0;
                for y in 0..cols {
                    if mat[i][y] == 1 {
                        col_ones += 1;
                    }
                }

                if row_ones == 1 && col_ones == 1 {
                    ret += 1;
                }
            }
        }
    }

    ret
}

fn main() {
    let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let ret = num_special(mat);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let mat = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        let ret = num_special(mat);
        assert_eq!(ret, 1);
    }
    {
        let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let ret = num_special(mat);
        assert_eq!(ret, 3);
    }
}
