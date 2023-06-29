fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut matrix = vec![vec![' '; cols]; rows];
    let mut all_keys = 0;

    let mut start_row = 0;
    let mut start_col = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            matrix[i][j] = c;

            if c.is_ascii_lowercase() {
                all_keys |= 1 << (c as usize - 'a' as usize);
            } else if c == '@' {
                start_row = i;
                start_col = j;
            }
        }
    }

    let mut q = VecDeque::new();

    q.push_back((start_row, start_col, 0usize));

    let moves = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut checked = HashMap::new();
    checked
        .entry(0)
        .or_insert(HashSet::new())
        .insert((start_row, start_col));

    let mut steps = 1;
    loop {
        let len = q.len();
        if len == 0 {
            return -1;
        }

        for _ in 0..len {
            let (row, col, keys) = q.pop_front().unwrap();

            for (x, y) in &moves {
                let r = row as i32 + x;
                let c = col as i32 + y;

                if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                    let r = r as usize;
                    let c = c as usize;

                    let cell = matrix[r][c];
                    if cell == '#' {
                        // wall
                        continue;
                    }

                    // key
                    if cell.is_lowercase() {
                        let key_bit = 1 << (cell as usize - 'a' as usize);
                        if keys & key_bit != 0 {
                            // already have key
                            continue;
                        }

                        let new_keys = keys | key_bit;
                        if new_keys == all_keys {
                            return steps;
                        }

                        checked
                            .entry(new_keys)
                            .or_insert(HashSet::new())
                            .insert((r, c));
                        q.push_back((r, c, new_keys));
                    }

                    // lock
                    if cell.is_ascii_uppercase() {
                        let key_bit = 1 << (cell.to_ascii_lowercase() as usize - 'a' as usize);
                        if keys & key_bit == 0 {
                            // don't have the key
                            continue;
                        }
                    }

                    if let Some(h) = checked.get_mut(&keys) {
                        if !h.contains(&(r, c)) {
                            h.insert((r, c));
                            q.push_back((r, c, keys));
                        }
                    }
                }
            }
        }

        steps += 1;
    }
}

fn main() {
    let grid = vec![
        "@.a..".to_string(),
        "###.#".to_string(),
        "b.A.B".to_string(),
    ];
    let ret = shortest_path_all_keys(grid);
    println!("ret={ret}");
}

#[test]
fn test_shortest_path_all_keys() {
    {
        let grid = vec![
            "@.a..".to_string(),
            "###.#".to_string(),
            "b.A.B".to_string(),
        ];
        let ret = shortest_path_all_keys(grid);
        assert_eq!(ret, 8);
    }
    {
        let grid = vec![
            "@..aA".to_string(),
            "..B#.".to_string(),
            "....b".to_string(),
        ];
        let ret = shortest_path_all_keys(grid);
        assert_eq!(ret, 6);
    }
    {
        let grid = vec!["@Aa".to_string()];
        let ret = shortest_path_all_keys(grid);
        assert_eq!(ret, -1);
    }
}
