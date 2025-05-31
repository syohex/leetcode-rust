fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let n = board.len();
    let mut cells = vec![(0, 0); n * n + 1];

    let mut is_even = true;
    let mut id = 1;
    for i in (0..n).rev() {
        if is_even {
            for j in 0..n {
                cells[id] = (i, j);
                id += 1;
            }
        } else {
            for j in (0..n).rev() {
                cells[id] = (i, j);
                id += 1;
            }
        }
        is_even = !is_even;
    }

    let mut min_rolls = vec![-1; n * n + 1];
    min_rolls[1] = 0;

    let mut q = VecDeque::new();
    q.push_back(1);

    while !q.is_empty() {
        let id = q.pop_front().unwrap();
        let limit = std::cmp::min(id + 6, n * n);

        for i in (id + 1)..=limit {
            let (row, col) = cells[i];
            let next = if board[row][col] == -1 {
                i
            } else {
                board[row][col] as usize
            };

            if min_rolls[next] == -1 {
                min_rolls[next] = min_rolls[id] + 1;
                q.push_back(next);
            }
        }
    }

    min_rolls[n * n]
}

fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    let ret = snakes_and_ladders(board);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        let ret = snakes_and_ladders(board);
        assert_eq!(ret, 4);
    }
    {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        let ret = snakes_and_ladders(board);
        assert_eq!(ret, 1);
    }
}
