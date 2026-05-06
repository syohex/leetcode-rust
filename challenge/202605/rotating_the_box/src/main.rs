fn rotate_the_box(boxx: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (rows, cols) = (boxx.len(), boxx[0].len());
    let mut ret = vec![vec!['?'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            ret[j][i] = boxx[i][j];
        }
    }

    let (rows, cols) = (cols, rows);
    for i in 0..rows {
        ret[i].reverse();
    }

    for j in 0..cols {
        let mut lowest = rows as i32 - 1;
        for i in (0..rows).rev() {
            if ret[i][j] == '#' {
                ret[i][j] = '.';
                ret[lowest as usize][j] = '#';
                lowest -= 1;
            } else if ret[i][j] == '*' {
                lowest = i as i32 - 1;
            }
        }
    }

    ret
}

fn main() {
    let box_grid = vec![vec!['#', '.', '#']];
    let ret = rotate_the_box(box_grid);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let box_grid = vec![vec!['#', '.', '#']];
        let expected = vec![vec!['.'], vec!['#'], vec!['#']];
        let ret = rotate_the_box(box_grid);
        assert_eq!(ret, expected);
    }
    {
        let box_grid = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let expected = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        let ret = rotate_the_box(box_grid);
        assert_eq!(ret, expected);
    }
    {
        let box_grid = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let expected = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];
        let ret = rotate_the_box(box_grid);
        assert_eq!(ret, expected);
    }
}
