fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    for row in grid.iter_mut() {
        row.sort_unstable();
    }

    let len = grid[0].len();
    let mut ret = 0;
    for i in 0..len {
        let mut max = 0;
        for row in grid.iter() {
            let v = row[len - 1 - i];
            max = std::cmp::max(max, v);
        }
        ret += max;
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 2, 4], vec![3, 3, 1]];
    let ret = delete_greatest_value(grid);
    println!("ret={ret}");
}

#[test]
fn test_delete_greatest_value() {
    {
        let grid = vec![vec![1, 2, 4], vec![3, 3, 1]];
        let ret = delete_greatest_value(grid);
        assert_eq!(ret, 8);
    }
    {
        let grid = vec![vec![10]];
        let ret = delete_greatest_value(grid);
        assert_eq!(ret, 10);
    }
}
