fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::{Ord, Ordering, PartialOrd};
    use std::collections::BinaryHeap;

    #[derive(Eq, PartialEq, Debug)]
    struct Data<'a> {
        row: usize,
        col: usize,
        grid: &'a [Vec<i32>],
    }

    impl<'a> Ord for Data<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.grid[other.row][other.col].cmp(&self.grid[self.row][self.col])
        }
    }

    impl<'a> PartialOrd for Data<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.grid[other.row][other.col].partial_cmp(&self.grid[self.row][self.col])
        }
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut q = BinaryHeap::new();
    q.push(Data {
        row: 0,
        col: 0,
        grid: &grid,
    });

    let mut checked = vec![vec![false; cols]; rows];
    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut ret = 0;
    while let Some(d) = q.pop() {
        checked[d.row][d.col] = true;

        ret = std::cmp::max(ret, grid[d.row][d.col]);
        if d.row == rows - 1 && d.col == cols - 1 {
            break;
        }

        for &(x, y) in &steps {
            let (r, c) = (d.row as i32 + x, d.col as i32 + y);
            if r >= 0
                && r < rows as i32
                && c >= 0
                && c < cols as i32
                && !checked[r as usize][c as usize]
            {
                q.push(Data {
                    row: r as usize,
                    col: c as usize,
                    grid: &grid,
                })
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    let ret = swim_in_water(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 2], vec![1, 3]];
        let ret = swim_in_water(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        let ret = swim_in_water(grid);
        assert_eq!(ret, 16);
    }
}
