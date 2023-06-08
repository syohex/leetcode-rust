fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    for row in grid {
        let mut left = 0i32;
        let mut right = row.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if row[mid as usize] < 0 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ret += row.len() as i32 - left;
    }

    ret
}

fn main() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    let ret = count_negatives(grid);
    println!("ret={ret}");
}

#[test]
fn test_count_negatives() {
    {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        let ret = count_negatives(grid);
        assert_eq!(ret, 8);
    }
    {
        let grid = vec![vec![3, 2], vec![1, 0]];
        let ret = count_negatives(grid);
        assert_eq!(ret, 0);
    }
}
