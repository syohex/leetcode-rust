fn largetst_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    fn check(i: usize, j: usize, len: usize, grid: &Vec<Vec<i32>>) -> bool {
        let mut row_sums = vec![0; len];
        let mut col_sums = vec![0; len];
        let mut diagnonal1 = 0;
        let mut diagnonal2 = 0;

        for x in 0..len {
            for y in 0..len {
                let v = grid[i+x][j+y];
                row_sums[x] += v;
                col_sums[y] += v;
                if x == y {
                    diagnonal1 += v;
                }
                if x + y == len - 1 {
                    diagnonal2 += v;
                }
            }
        }

        diagnonal1 == diagnonal2
        && row_sums.iter().all(|&v| v == diagnonal1)
        && col_sums.iter().all(|&v| v == diagnonal1)
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let max_len = std::cmp::min(rows, cols);
    for len in (2..=max_len).rev() {
        for i in 0..=(rows - len) {
            for j in 0..=(cols - len) {
                if check(i, j, len, &grid) {
                    return len as i32;
                }
            }
        }
    }

    1
}

fn main() {
    let grid = vec![
        vec![7, 1, 4, 5, 6],
        vec![2, 5, 1, 6, 4],
        vec![1, 5, 4, 3, 2],
        vec![1, 2, 7, 3, 4],
    ];
    let ret = largetst_magic_square(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4],
        ];
        let ret = largetst_magic_square(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec![5, 1, 3, 1], vec![9, 3, 3, 1], vec![1, 3, 3, 8]];
        let ret = largetst_magic_square(grid);
        assert_eq!(ret, 2);
    }
}
