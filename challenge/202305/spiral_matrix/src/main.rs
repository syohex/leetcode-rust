fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    enum State {
        Right,
        Down,
        Left,
        Up,
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut ret = vec![];
    let mut matrix = matrix;
    let mut state = State::Right;
    let mut steps = 0;

    let mut row = 0;
    let mut col = 0;
    loop {
        if steps >= rows * cols {
            break;
        }

        ret.push(matrix[row][col]);
        steps += 1;
        matrix[row][col] = std::i32::MIN;

        match state {
            State::Right => {
                if col + 1 < cols && matrix[row][col + 1] != std::i32::MIN {
                    col += 1;
                } else {
                    state = State::Down;
                    row += 1;
                }
            }
            State::Down => {
                if row + 1 < rows && matrix[row + 1][col] != std::i32::MIN {
                    row += 1;
                } else {
                    state = State::Left;
                    col -= 1;
                }
            }
            State::Left => {
                if col as i32 - 1 >= 0 && matrix[row][col - 1] != std::i32::MIN {
                    col -= 1;
                } else {
                    state = State::Up;
                    row -= 1;
                }
            }
            State::Up => {
                if row as i32 - 1 >= 0 && matrix[row - 1][col] != std::i32::MIN {
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
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = spiral_order(matrix);
    println!("ret={ret:?}");
}

#[test]
fn test_spiral_order() {
    {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        let ret = spiral_order(matrix);
        assert_eq!(ret, expected);
    }
    {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        let ret = spiral_order(matrix);
        assert_eq!(ret, expected);
    }
    {
        let matrix = vec![vec![9]];
        let expected = vec![9];
        let ret = spiral_order(matrix);
        assert_eq!(ret, expected);
    }
    {
        let matrix = vec![vec![1, 2, 3]];
        let expected = vec![1, 2, 3];
        let ret = spiral_order(matrix);
        assert_eq!(ret, expected);
    }
}
