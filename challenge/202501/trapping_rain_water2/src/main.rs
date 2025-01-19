fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    use std::cmp::{max, Reverse};
    use std::collections::BinaryHeap;

    let (rows, cols) = (height_map.len(), height_map[0].len());
    let mut visited = vec![vec![false; cols]; rows];

    let mut q = BinaryHeap::new();
    for i in 0..rows {
        if i == 0 || i == rows - 1 {
            for j in 0..cols {
                q.push((Reverse(height_map[i][j]), (i, j)));
                visited[i][j] = true;
            }
        } else {
            q.push((Reverse(height_map[i][0]), (i, 0)));
            q.push((Reverse(height_map[i][cols - 1]), (i, cols - 1)));

            visited[i][0] = true;
            visited[i][cols - 1] = true;
        }
    }

    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut ret = 0;
    while let Some((Reverse(height), (row, col))) = q.pop() {
        for (x, y) in &steps {
            let (r, c) = (row as i32 + x, col as i32 + y);
            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                let (r, c) = (r as usize, c as usize);
                if !visited[r][c] {
                    visited[r][c] = true;

                    if height_map[r][c] < height {
                        ret += height - height_map[r][c];
                    }

                    let h = max(height_map[r][c], height);
                    q.push((Reverse(h), (r, c)));
                }
            }
        }
    }

    ret
}

fn main() {
    let height_map = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1],
    ];
    let ret = trap_rain_water(height_map);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let ret = trap_rain_water(height_map);
        assert_eq!(ret, 4);
    }
    {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let ret = trap_rain_water(height_map);
        assert_eq!(ret, 10);
    }
}
