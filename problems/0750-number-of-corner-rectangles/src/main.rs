pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() <= 1 {
        return 0;
    }

    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                for k in (j + 1)..grid[i].len() {
                    if grid[i][k] == 1 {
                        for m in (i + 1)..grid.len() {
                            if grid[m][k] == 1 && grid[m][j] == 1 {
                                ret += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    ret
}

fn main() {
    let input: Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
    ];

    println!(
        "count_corner_rectangles({:?})={}",
        input.clone(),
        count_corner_rectangles(input)
    );
}

#[test]
fn test_count_corner_rectangles() {
    {
        let input: Vec<Vec<i32>> = vec![
            vec![1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
        ];

        assert_eq!(count_corner_rectangles(input), 1);
    }
    {
        let input: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        assert_eq!(count_corner_rectangles(input), 9);
    }
    {
        let input: Vec<Vec<i32>> = vec![vec![1, 1, 1, 1]];

        assert_eq!(count_corner_rectangles(input), 0);
    }
}
