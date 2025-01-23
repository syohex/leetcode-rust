fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut row_sum = vec![0; rows];
    let mut col_sum = vec![0; cols];

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                row_sum[i] += 1;
                col_sum[j] += 1;
            }
        }
    }

    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 && (row_sum[i] >= 2 || col_sum[j] >= 2) {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 0], vec![1, 1]];
    let ret = count_servers(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let ret = count_servers(grid);
        assert_eq!(ret, 0);
    }
    {
        let grid = vec![vec![1, 0], vec![1, 1]];
        let ret = count_servers(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let ret = count_servers(grid);
        assert_eq!(ret, 4);
    }
}
