fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let rows = mat.len();
    let cols = mat[0].len();
    let mut ret = vec![vec![-1; cols]; rows];

    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 0 {
                ret[i][j] = 0;
                q.push_back((i, j));
                visited[i][j] = true;
            }
        }
    }

    let moves = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut steps = 0;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let (r, c) = q.pop_front().unwrap();
            if ret[r][c] == -1 {
                ret[r][c] = steps;
            }

            for (x, y) in &moves {
                let new_x = r as i32 + *x;
                let new_y = c as i32 + *y;
                if new_x >= 0
                    && new_x < rows as i32
                    && new_y >= 0
                    && new_y < cols as i32
                    && !visited[new_x as usize][new_y as usize]
                {
                    visited[new_x as usize][new_y as usize] = true;
                    q.push_back((new_x as usize, new_y as usize));
                }
            }
        }

        steps += 1;
    }

    ret
}

fn main() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let ret = update_matrix(mat);
    println!("ret={ret:?}");
}

#[test]
fn test_update_matrix() {
    {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let ret = update_matrix(mat);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        let ret = update_matrix(mat);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
        ];
        let expected = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0],
        ];
        let ret = update_matrix(mat);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![
            vec![0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
        ];
        let expected = vec![
            vec![0, 1, 0, 1, 2],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
        ];
        let ret = update_matrix(mat);
        assert_eq!(ret, expected);
    }
}
