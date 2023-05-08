fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();

    let mut ret = 0;
    for i in 0..n {
        ret += mat[i][i];
        ret += mat[n - 1 - i][i];
    }

    if n % 2 == 1 {
        let i = n / 2;
        ret -= mat[i][i];
    }

    ret
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = diagonal_sum(mat);
    println!("ret={ret}");
}

#[test]
fn test_diagonal_sum() {
    {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ret = diagonal_sum(mat);
        assert_eq!(ret, 25);
    }
    {
        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        let ret = diagonal_sum(mat);
        assert_eq!(ret, 8);
    }
    {
        let mat = vec![vec![5]];
        let ret = diagonal_sum(mat);
        assert_eq!(ret, 5);
    }
}
