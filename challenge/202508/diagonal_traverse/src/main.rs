fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![];

    let (rows, cols) = (mat.len() as i32, mat[0].len() as i32);
    let mut row = 0;
    let mut col = 0;
    let mut up_direction = true;

    while row < rows && col < cols {
        ret.push(mat[row as usize][col as usize]);

        let (r, c) = if up_direction {
            (row - 1, col + 1)
        } else {
            (row + 1, col - 1)
        };

        if r < 0 || r >= rows || c < 0 || c >= cols {
            if up_direction {
                row = if c == cols { row + 1 } else { row };
                col = if c < cols { col + 1 } else { col }
            } else {
                row = if r < rows { row + 1 } else { row };
                col = if r == rows { col + 1 } else { col }
            }

            up_direction = !up_direction;
        } else {
            row = r;
            col = c;
        }
    }
    ret
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = find_diagonal_order(mat);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        let ret = find_diagonal_order(mat);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![1, 2, 3, 4];
        let ret = find_diagonal_order(mat);
        assert_eq!(ret, expected);
    }
}
