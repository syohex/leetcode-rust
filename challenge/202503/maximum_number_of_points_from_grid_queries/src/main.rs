fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut queries: Vec<_> = queries
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    queries.sort_unstable();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    visited[0][0] = true;

    let mut q = BinaryHeap::new();
    q.push((Reverse(grid[0][0]), 0, 0));

    let mut ret = vec![0; queries.len()];
    let mut points = 0;
    for (query, i) in queries {
        while !q.is_empty() {
            if let Some((Reverse(v), _, _)) = q.peek() {
                if *v >= query {
                    break;
                }
            }

            let (_, row, col) = q.pop().unwrap();
            points += 1;

            for &(x, y) in &steps {
                let r = row + x;
                let c = col + y;

                if r >= 0 && r < rows && c >= 0 && c < cols && !visited[r as usize][c as usize] {
                    q.push((Reverse(grid[r as usize][c as usize]), r, c));
                    visited[r as usize][c as usize] = true;
                }
            }
        }

        ret[i] = points;
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
    let queries = vec![5, 6, 2];
    let ret = max_points(grid, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
        let queries = vec![5, 6, 2];
        let expected = vec![5, 8, 1];
        let ret = max_points(grid, queries);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![5, 2, 1], vec![1, 1, 2]];
        let queries = vec![3];
        let expected = vec![0];
        let ret = max_points(grid, queries);
        assert_eq!(ret, expected);
    }
}
