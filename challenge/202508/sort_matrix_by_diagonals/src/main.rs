fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::Reverse;

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut ret = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let mut v = vec![];
        let mut row = i;
        let mut col = 0;
        while row < rows && col < cols {
            v.push(grid[row][col]);
            row += 1;
            col += 1;
        }

        v.sort_unstable_by_key(|&n| Reverse(n));

        row = i;
        col = 0;
        for n in v {
            ret[row][col] = n;
            row += 1;
            col += 1;
        }
    }

    for i in 1..cols {
        let mut v = vec![];
        let mut row = 0;
        let mut col = i;
        while row < rows && col < cols {
            v.push(grid[row][col]);
            row += 1;
            col += 1;
        }

        v.sort_unstable();

        row = 0;
        col = i;
        for n in v {
            ret[row][col] = n;
            row += 1;
            col += 1;
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]];
    let ret = sort_matrix(grid);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]];
        let expected = vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]];
        let ret = sort_matrix(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![0, 1], vec![1, 2]];
        let expected = vec![vec![2, 1], vec![1, 0]];
        let ret = sort_matrix(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1]];
        let expected = vec![vec![1]];
        let ret = sort_matrix(grid);
        assert_eq!(ret, expected);
    }
}
