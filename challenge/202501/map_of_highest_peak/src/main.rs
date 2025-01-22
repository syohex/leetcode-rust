fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let (rows, cols) = (is_water.len(), is_water[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut ret = vec![vec![i32::MAX; cols]; rows];
    let mut q = BinaryHeap::new();

    for i in 0..rows {
        for j in 0..cols {
            if is_water[i][j] == 1 {
                ret[i][j] = 0;
                visited[i][j] = true;
                q.push((Reverse(0), i, j));
            }
        }
    }

    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((Reverse(height), row, col)) = q.pop() {
        for &(x, y) in &steps {
            let r = row as i32 + x;
            let c = col as i32 + y;

            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                let (r, c) = (r as usize, c as usize);
                if !visited[r][c] {
                    visited[r][c] = true;
                    ret[r][c] = height + 1;
                    q.push((Reverse(height + 1), r, c));
                }
            }
        }
    }

    ret
}

fn main() {
    let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
    let ret = highest_peak(is_water);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let is_water = vec![vec![0, 1], vec![0, 0]];
        let expected = vec![vec![1, 0], vec![2, 1]];
        let ret = highest_peak(is_water);
        assert_eq!(ret, expected);
    }
    {
        let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        let expected = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]];
        let ret = highest_peak(is_water);
        assert_eq!(ret, expected);
    }
}
