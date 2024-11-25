fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    let goal = vec![1, 2, 3, 4, 5, 0];
    let moves = [
        vec![1, 3],
        vec![0, 2, 4],
        vec![1, 5],
        vec![0, 4],
        vec![1, 3, 5],
        vec![2, 4],
    ];

    let mut init = vec![];
    for row in board {
        for n in row {
            init.push(n);
        }
    }

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(init);

    let mut steps = 0;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let v = q.pop_front().unwrap();
            if v == goal {
                return steps;
            }

            visited.insert(v.clone());

            let zero_pos = v.iter().position(|v| *v == 0).unwrap();
            for next in &moves[zero_pos] {
                let mut tmp = v.clone();
                tmp.swap(zero_pos, *next);

                if !visited.contains(&tmp) {
                    q.push_back(tmp);
                }
            }
        }

        steps += 1;
    }

    -1
}

fn main() {
    let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
    let ret = sliding_puzzle(board);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
        let ret = sliding_puzzle(board);
        assert_eq!(ret, 1);
    }
    {
        let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
        let ret = sliding_puzzle(board);
        assert_eq!(ret, -1);
    }
    {
        let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
        let ret = sliding_puzzle(board);
        assert_eq!(ret, 5);
    }
}
