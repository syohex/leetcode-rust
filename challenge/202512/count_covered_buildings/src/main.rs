fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut row_groups: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut col_groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for b in buildings {
        let (row, col) = (b[0] as usize, b[1] as usize);
        row_groups.entry(row).or_insert(vec![]).push(col);
        col_groups.entry(col).or_insert(vec![]).push(row);
    }

    for v in row_groups.values_mut() {
        v.sort_unstable();
    }
    for v in col_groups.values_mut() {
        v.sort_unstable();
    }

    let mut ret = 0;
    for (row, cols) in row_groups.into_iter().filter(|(_, cols)| cols.len() >= 3) {
        let len = cols.len();
        for col in cols.into_iter().take(len - 1).skip(1) {
            if let Some(rows) = col_groups.get(&col)
                && rows.len() >= 3
                && let Ok(pos) = rows.binary_search(&row)
                && (pos != 0 && pos != rows.len() - 1)
            {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let buildings = vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]];
    let ret = count_covered_buildings(3, buildings);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let buildings = vec![
            vec![2, 4],
            vec![1, 2],
            vec![3, 1],
            vec![1, 4],
            vec![2, 3],
            vec![3, 3],
            vec![2, 2],
            vec![1, 3],
        ];
        let ret = count_covered_buildings(4, buildings);
        assert_eq!(ret, 1);
    }
    {
        let buildings = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 1],
            vec![2, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 2],
        ];
        let ret = count_covered_buildings(3, buildings);
        assert_eq!(ret, 1);
    }
    {
        let buildings = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 1],
            vec![1, 1],
            vec![2, 3],
            vec![1, 3],
        ];
        let ret = count_covered_buildings(3, buildings);
        assert_eq!(ret, 0);
    }
    {
        let buildings = vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]];
        let ret = count_covered_buildings(3, buildings);
        assert_eq!(ret, 1);
    }
    {
        let buildings = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];
        let ret = count_covered_buildings(3, buildings);
        assert_eq!(ret, 0);
    }
    {
        let buildings = vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]];
        let ret = count_covered_buildings(3, buildings);
        assert_eq!(ret, 1);
    }
}
