fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut row_computers = vec![0; rows];
    let mut col_computers = vec![0; cols];
    let mut computers = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                row_computers[i] += 1;
                col_computers[j] += 1;
                computers += 1;
            }
        }
    }

    let mut isolated = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 && row_computers[i] == 1 && col_computers[j] == 1 {
                isolated += 1;
            }
        }
    }

    computers - isolated
}

fn main() {
    let grid = vec![
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
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
