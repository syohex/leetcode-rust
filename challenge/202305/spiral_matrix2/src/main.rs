fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    enum State {
        Right,
        Down,
        Left,
        Up,
    }

    let n = n as usize;
    let limit = n * n;
    let mut ret = vec![vec![0; n]; n];
    let mut steps = 1;
    let mut state = State::Right;
    let mut row = 0usize;
    let mut col = 0usize;

    while steps <= limit {
        ret[row][col] = steps as i32;
        steps += 1;

        match state {
            State::Right => {
                if col + 1 < n && ret[row][col + 1] == 0 {
                    col += 1;
                } else {
                    state = State::Down;
                    row += 1;
                }
            }
            State::Down => {
                if row + 1 < n && ret[row + 1][col] == 0 {
                    row += 1;
                } else {
                    state = State::Left;
                    col -= 1;
                }
            }
            State::Left => {
                if col >= 1 && ret[row][col - 1] == 0 {
                    col -= 1;
                } else {
                    state = State::Up;
                    row -= 1;
                }
            }
            State::Up => {
                if row >= 1 && ret[row - 1][col] == 0 {
                    row -= 1;
                } else {
                    state = State::Right;
                    col += 1;
                }
            }
        }
    }

    ret
}

fn main() {
    let ret = generate_matrix(3);
    println!("ret={ret:?}");
}

#[test]
fn test_generate_matrix() {
    {
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let ret = generate_matrix(3);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![vec![1]];
        let ret = generate_matrix(1);
        assert_eq!(ret, expected);
    }
}
