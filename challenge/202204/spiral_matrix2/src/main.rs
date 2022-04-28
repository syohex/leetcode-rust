fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let len = n as usize;
    let mut ret = vec![vec![-1; len]; len];

    let mut row = 0;
    let mut col = 0;
    let mut count = 1;
    let limit = n * n;

    enum Direction {
        Right,
        Down,
        Left,
        Up,
    }

    let mut dir = Direction::Right;

    while count <= limit {
        ret[row][col] = count;
        count += 1;

        match dir {
            Direction::Right => {
                if col + 1 <= len - 1 && ret[row][col + 1] == -1 {
                    col += 1;
                } else {
                    row += 1;
                    dir = Direction::Down;
                }
            }
            Direction::Down => {
                if row + 1 <= len - 1 && ret[row + 1][col] == -1 {
                    row += 1;
                } else {
                    col -= 1;
                    dir = Direction::Left;
                }
            }
            Direction::Left => {
                if col >= 1 && ret[row][col - 1] == -1 {
                    col -= 1;
                } else {
                    row -= 1;
                    dir = Direction::Up;
                }
            }
            Direction::Up => {
                if row >= 1 && ret[row - 1][col] == -1 {
                    row -= 1;
                } else {
                    col += 1;
                    dir = Direction::Right;
                }
            }
        }
    }

    ret
}

fn main() {
    let ret = generate_matrix(4);
    println!("ret={:?}", ret);

    let ret = generate_matrix(2);
    println!("ret={:?}", ret);
}

#[test]
fn test_generate_matrix() {
    {
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let ret = generate_matrix(3);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ];
        let ret = generate_matrix(4);
        assert_eq!(ret, expected);
    }
}
