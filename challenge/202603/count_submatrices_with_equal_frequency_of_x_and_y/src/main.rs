fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut acc = vec![vec![(0, 0); cols + 1]; rows + 1];
    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            let x = acc[i + 1][j].0 + acc[i][j + 1].0 - acc[i][j].0;
            let y = acc[i + 1][j].1 + acc[i][j + 1].1 - acc[i][j].1;
            let (new_x, new_y) = match grid[i][j] {
                'X' => (x + 1, y),
                'Y' => (x, y + 1),
                _ => (x, y),
            };

            if new_x != 0 && new_x == new_y {
                ret += 1;
            }

            acc[i + 1][j + 1] = (new_x, new_y);
        }
    }

    ret
}

fn main() {
    let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
    let ret = number_of_submatrices(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        let ret = number_of_submatrices(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec!['X', 'X'], vec!['X', 'Y']];
        let ret = number_of_submatrices(grid);
        assert_eq!(ret, 0);
    }
    {
        let grid = vec![vec!['.', '.'], vec!['.', '.']];
        let ret = number_of_submatrices(grid);
        assert_eq!(ret, 0);
    }
}
