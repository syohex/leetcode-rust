fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let rows = board.len();
    let cols = board[0].len();
    let mut tmp = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut lives = 0;
            if i >= 1 {
                if j >= 1 {
                    lives += board[i - 1][j - 1];
                }
                lives += board[i - 1][j];
                if j as i32 <= cols as i32 - 2 {
                    lives += board[i - 1][j + 1];
                }
            }

            if j >= 1 {
                lives += board[i][j - 1];
            }
            if j as i32 <= cols as i32 - 2 {
                lives += board[i][j + 1];
            }
            if i as i32 <= rows as i32 - 2 {
                if j >= 1 {
                    lives += board[i + 1][j - 1];
                }
                lives += board[i + 1][j];
                if j as i32 <= cols as i32 - 2 {
                    lives += board[i + 1][j + 1];
                }
            }

            tmp[i][j] = board[i][j];

            if board[i][j] == 1 {
                if lives < 2 || lives > 3 {
                    tmp[i][j] = 0;
                }
            } else {
                if lives == 3 {
                    tmp[i][j] = 1;
                }
            }
        }
    }

    *board = tmp;
}

fn main() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
    game_of_life(&mut board);
    println!("ret={:?}", board);
}

#[test]
fn test_game_of_life() {
    {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        game_of_life(&mut board);
        assert_eq!(board, expected);
    }
    {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        let expected = vec![vec![1, 1], vec![1, 1]];
        game_of_life(&mut board);
        assert_eq!(board, expected);
    }
}
