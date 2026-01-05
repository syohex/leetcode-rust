fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut sum = 0i64;
    let mut negatives = 0;
    let mut min = i64::MAX;

    for v in matrix {
        for n in v {
            if n < 0 {
                negatives += 1;
            }

            let a = n.abs() as i64;
            sum += a;
            min = std::cmp::min(min, a);
        }
    }

    if negatives % 2 == 0 {
        sum
    } else {
        sum - 2 * min
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
    let ret = max_matrix_sum(matrix);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let ret = max_matrix_sum(matrix);
        assert_eq!(ret, 4);
    }
    {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        let ret = max_matrix_sum(matrix);
        assert_eq!(ret, 16);
    }
}
