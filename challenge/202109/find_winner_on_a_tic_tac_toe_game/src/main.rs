fn tictoctoe(moves: Vec<Vec<i32>>) -> String {
    #[derive(Clone, PartialEq, Eq, Debug)]
    enum Value {
        A,
        B,
        Empty,
    }

    let mut board = vec![vec![Value::Empty; 3]; 3];
    for (i, m) in moves.iter().enumerate() {
        board[m[0] as usize][m[1] as usize] = if i % 2 == 0 { Value::A } else { Value::B }
    }

    let check = |v: Value| -> bool {
        for i in 0..3 {
            if board[i][0] == v && board[i][1] == v && board[i][2] == v {
                return true;
            }
            if board[0][i] == v && board[1][i] == v && board[2][i] == v {
                return true;
            }
        }

        if board[0][0] == v && board[1][1] == v && board[2][2] == v {
            return true;
        }
        if board[2][0] == v && board[1][1] == v && board[0][2] == v {
            return true;
        }

        false
    };

    if check(Value::A) {
        "A".to_string()
    } else if check(Value::B) {
        "B".to_string()
    } else if moves.len() == 9 {
        "Draw".to_string()
    } else {
        "Pending".to_string()
    }
}

fn main() {
    let moves = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
    let ret = tictoctoe(moves);
    println!("ret={}", ret);
}

#[test]
fn test_tictoctoe() {
    {
        let moves = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
        let ret = tictoctoe(moves);
        assert_eq!(ret, "A");
    }
    {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
        ];
        let ret = tictoctoe(moves);
        assert_eq!(ret, "B");
    }
    {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2],
        ];
        let ret = tictoctoe(moves);
        assert_eq!(ret, "Draw");
    }
    {
        let moves = vec![vec![0, 0], vec![1, 1]];
        let ret = tictoctoe(moves);
        assert_eq!(ret, "Pending");
    }
}
