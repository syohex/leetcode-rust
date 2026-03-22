fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();
    let mut mat = mat;

    for _ in 0..4 {
        let limit = n / 2;
        let limit2 = n.div_ceil(2);
        for i in 0..limit {
            for j in 0..limit2 {
                let tmp = mat[i][j];
                mat[i][j] = mat[n - 1 - j][i];
                mat[n - 1 - j][i] = mat[n-1-i][n-1-j];
                mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
                mat[j][n - 1 - i] = tmp;
            }
        }

        if mat == target {
            return true;
        }
    }

    false
}

fn main() {
    let mat = vec![vec![0, 1], vec![1, 0]];
    let target = vec![vec![1, 0], vec![0, 1]];
    println!("ret={}", find_rotation(mat, target));
}

#[test]
fn test() {
    {
        let mat = vec![vec![0, 1], vec![1, 0]];
        let target = vec![vec![1, 0], vec![0, 1]];
        assert!(find_rotation(mat, target));
    }
    {
        let mat = vec![vec![0, 1], vec![1, 1]];
        let target = vec![vec![1, 0], vec![0, 1]];
        assert!(!find_rotation(mat, target));
    }
    {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let target = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
        assert!(find_rotation(mat, target));
    }
}
