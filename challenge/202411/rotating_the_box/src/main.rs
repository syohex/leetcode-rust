fn rotate_the_box(box2: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut v = box2;
    let (rows, cols) = (v.len(), v[0].len());
    for x in v.iter_mut() {
        let mut limit = cols;
        for i in (0..cols).rev() {
            if x[i] == '#' {
                let mut pos = i;
                for j in (i + 1)..limit {
                    match x[j] {
                        '.' => pos = j,
                        _ => break,
                    }
                }

                if i != pos {
                    x.swap(i, pos);
                    limit = pos;
                } else {
                    limit = i;
                }
            }
        }
    }

    let mut ret = vec![vec!['\0'; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            ret[j][rows - 1 - i] = v[i][j];
        }
    }

    ret
}

fn main() {
    let b = vec![
        vec!['#', '#', '*', '.', '*', '.'],
        vec!['#', '#', '#', '*', '.', '.'],
        vec!['#', '#', '#', '.', '#', '.'],
    ];
    let ret = rotate_the_box(b);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let b = vec![vec!['#', '.', '#']];
        let expected = vec![vec!['.'], vec!['#'], vec!['#']];
        let ret = rotate_the_box(b);
        assert_eq!(ret, expected);
    }
    {
        let b = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let expected = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        let ret = rotate_the_box(b);
        assert_eq!(ret, expected);
    }
    {
        let b = vec![
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
        let ret = rotate_the_box(b);
        assert_eq!(ret, expected);
    }
}
