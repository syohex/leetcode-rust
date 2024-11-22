fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let mut ret = 0;
    for i in 0..rows {
        let inverted: Vec<i32> = matrix[i]
            .iter()
            .map(|n| if *n == 1 { 0 } else { 1 })
            .collect();

        let mut tmp = 1;
        for j in (i + 1)..rows {
            if matrix[i] == matrix[j] || matrix[j] == inverted {
                tmp += 1;
            }
        }

        ret = std::cmp::max(ret, tmp);
    }

    ret
}

fn main() {
    let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    let ret = max_equal_rows_after_flips(matrix);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let matrix = vec![vec![0, 1], vec![1, 1]];
        let ret = max_equal_rows_after_flips(matrix);
        assert_eq!(ret, 1);
    }
    {
        let matrix = vec![vec![0, 1], vec![1, 0]];
        let ret = max_equal_rows_after_flips(matrix);
        assert_eq!(ret, 2);
    }
    {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
        let ret = max_equal_rows_after_flips(matrix);
        assert_eq!(ret, 2);
    }
}
