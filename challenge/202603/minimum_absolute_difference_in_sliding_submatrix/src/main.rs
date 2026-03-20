fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    let k = k as usize;
    let mut ret = vec![vec![0; grid[0].len() - k + 1]; grid.len() - k + 1];

    let (rows, cols) = (ret.len(), ret[0].len());
    for i in 0..rows {
        for j in 0..cols {
            let mut s = HashSet::new();
            for x in 0..k {
                for y in 0..k {
                    s.insert(grid[i + x][j + y]);
                }
            }

            if s.len() == 1 {
                ret[i][j] = 0;
            } else {
                let mut v: Vec<_> = s.into_iter().collect();
                v.sort_unstable();
                let mut min = i32::MAX;
                for x in 1..v.len() {
                    min = std::cmp::min(min, (v[x] - v[x - 1]).abs());
                }

                ret[i][j] = min;
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 8], vec![3, -2]];
    let ret = min_abs_diff(grid, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 8], vec![3, -2]];
        let expected = vec![vec![2]];
        let ret = min_abs_diff(grid, 2);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![3, -1]];
        let expected = vec![vec![0, 0]];
        let ret = min_abs_diff(grid, 1);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1, -2, 3], vec![2, 3, 5]];
        let expected = vec![vec![1, 2]];
        let ret = min_abs_diff(grid, 2);
        assert_eq!(ret, expected);
    }
}
