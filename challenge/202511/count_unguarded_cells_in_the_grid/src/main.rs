fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum State {
        Unguarded,
        Wall,
        Guard,
        Guarded,
    }

    let mut grid = vec![vec![State::Unguarded; n as usize]; m as usize];
    for g in &guards {
        grid[g[0] as usize][g[1] as usize] = State::Guard;
    }
    for w in walls {
        grid[w[0] as usize][w[1] as usize] = State::Wall;
    }

    for g in guards {
        for i in (0..g[0]).rev() {
            match grid[i as usize][g[1] as usize] {
                State::Wall | State::Guard => break,
                _ => grid[i as usize][g[1] as usize] = State::Guarded,
            }
        }

        for i in (g[0] + 1)..m {
            match grid[i as usize][g[1] as usize] {
                State::Wall | State::Guard => break,
                _ => grid[i as usize][g[1] as usize] = State::Guarded,
            }
        }

        for i in (0..g[1]).rev() {
            match grid[g[0] as usize][i as usize] {
                State::Wall | State::Guard => break,
                _ => grid[g[0] as usize][i as usize] = State::Guarded,
            }
        }

        for i in (g[1] + 1)..n {
            match grid[g[0] as usize][i as usize] {
                State::Wall | State::Guard => break,
                _ => grid[g[0] as usize][i as usize] = State::Guarded,
            }
        }
    }

    grid.into_iter().fold(0, |acc, v| {
        acc + v.into_iter().filter(|s| *s == State::Unguarded).count() as i32
    })
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
        let m = 2;
        let n = 7;
        let guards = vec![vec![1, 5], vec![1, 1], vec![1, 6], vec![0, 2]];
        let walls = vec![vec![0, 6], vec![0, 3], vec![0, 5]];
        let ret = count_unguarded(m, n, guards, walls);
        assert_eq!(ret, 1);
    }
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
