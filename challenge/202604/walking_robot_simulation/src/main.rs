fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let mut blocks = HashSet::new();
    for ob in obstacles {
        blocks.insert((ob[0], ob[1]));
    }

    let steps = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut current_direction = 0;

    let mut row = 0;
    let mut col = 0;
    let mut ret = 0;
    for cmd in commands {
        match cmd {
            -1 => current_direction = (current_direction + 1) % 4,
            -2 => current_direction = (current_direction + 3) % 4,
            n => {
                for _ in 0..n {
                    let new_row = row + steps[current_direction].0;
                    let new_col = col + steps[current_direction].1;
                    if blocks.contains(&(new_row, new_col)) {
                        break;
                    }

                    row = new_row;
                    col = new_col;
                }

                ret = std::cmp::max(ret, row * row + col * col);
            }
        }
    }

    ret
}

fn main() {
    let ret = robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(robot_sim(vec![4, -1, 3], vec![]), 25);
    assert_eq!(robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
    assert_eq!(robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]), 36);
}
