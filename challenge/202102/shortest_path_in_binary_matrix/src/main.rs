use std::collections::VecDeque;

fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let mut distances: Vec<Vec<i32>> = grid.clone();
    let mut queue: VecDeque<[i32; 2]> = VecDeque::new();

    if grid[0][0] == 1 {
        return -1;
    }

    queue.push_back([0, 0]);
    distances[0][0] = 1;

    let row_limit: i32 = (grid.len() - 1) as i32;
    let col_limit: i32 = (grid[0].len() - 1) as i32;
    while !queue.is_empty() {
        let d = queue.pop_front().unwrap();
        let distance = distances[d[0] as usize][d[1] as usize];
        if d[0] == row_limit && d[1] == col_limit {
            return distance;
        }

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x: i32 = d[0] + i;
                let y: i32 = d[1] + j;
                if x >= 0
                    && x <= row_limit
                    && y >= 0
                    && y <= col_limit
                    && distances[x as usize][y as usize] == 0
                {
                    queue.push_back([x, y]);
                    distances[x as usize][y as usize] = distance + 1;
                }
            }
        }
    }

    -1
}

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    println!("answer={}", shortest_path_binary_matrix(grid));
}

#[test]
fn test_shortest_path_binary_matrix() {
    {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 2);
    }
    {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 4);
    }
    {
        let grid = vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), 7);
    }
}
