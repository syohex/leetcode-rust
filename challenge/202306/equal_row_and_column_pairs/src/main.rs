fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut v = vec![];

    for i in 0..n {
        let mut tmp = vec![];
        for j in 0..n {
            tmp.push(grid[j][i]);
        }
        v.push(tmp);
    }

    let mut ret = 0;
    let m = v.len();
    for i in 0..n {
        for j in 0..m {
            if grid[i] == v[j] {
                ret += 1;
            }
        }
    }

    ret
}
fn main() {
    let grid = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];
    let ret = equal_pairs(grid);
    println!("ret={ret}");
}

#[test]
fn test_equal_pairs() {
    {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        let ret = equal_pairs(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        let ret = equal_pairs(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec![13, 13], vec![13, 13]];
        let ret = equal_pairs(grid);
        assert_eq!(ret, 4);
    }
}
