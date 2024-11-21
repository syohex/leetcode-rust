fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    #[derive(Debug, Clone, Eq, PartialEq)]
    enum State {
        UnGuarded,
        Wall,
        Guarded,
        Guard,
    }

    let (m, n) = (m as usize, n as usize);
    let mut matrix = vec![vec![State::UnGuarded; n]; m];

    for wall in &walls {
        matrix[wall[0] as usize][wall[1] as usize] = State::Wall;
    }
    for guard in &guards {
        matrix[guard[0] as usize][guard[1] as usize] = State::Guard;
    }

    for guard in &guards {
        let (row, col) = (guard[0] as usize, guard[1] as usize);

        let mut i = row as i32 - 1;
        while i >= 0 {
            match matrix[i as usize][col] {
                State::Wall | State::Guard => break,
                _ => matrix[i as usize][col] = State::Guarded,
            }
            i -= 1;
        }

        for i in row + 1..m {
            match matrix[i][col] {
                State::Wall | State::Guard => break,
                _ => matrix[i][col] = State::Guarded,
            }
        }

        let mut j = col as i32 - 1;
        while j >= 0 {
            match matrix[row][j as usize] {
                State::Wall | State::Guard => break,
                _ => matrix[row][j as usize] = State::Guarded,
            }
            j -= 1;
        }
        for i in col + 1..n {
            match matrix[row][i] {
                State::Wall | State::Guard => break,
                _ => matrix[row][i] = State::Guarded,
            }
        }
    }

    let mut ret = 0;
    for row in &matrix {
        for n in row {
            if *n == State::UnGuarded {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let m = 4;
    let n = 6;
    let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
    let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
    let ret = count_unguarded(m, n, guards, walls);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let m = 4;
        let n = 6;
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        let ret = count_unguarded(m, n, guards, walls);
        assert_eq!(ret, 7);
    }
    {
        let m = 3;
        let n = 3;
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        let ret = count_unguarded(m, n, guards, walls);
        assert_eq!(ret, 4);
    }
}
