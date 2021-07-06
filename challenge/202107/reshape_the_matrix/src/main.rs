fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let r = r as usize;
    let c = c as usize;
    let orig_size = mat.len() * mat[0].len();
    let reshaped_size = r * c;
    if orig_size != reshaped_size {
        mat
    } else {
        let mut ret = vec![vec![0; c]; r];
        let mut index = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                let row = index / c;
                let col = index % c;
                ret[row][col] = mat[i][j];

                index += 1;
            }
        }
        ret
    }
}

fn main() {
    let mat = vec![vec![1, 2], vec![3, 4]];
    let ret = matrix_reshape(mat, 1, 4);
    println!("ret={:?}", ret);
}

#[test]
fn test_matrix_reshape() {
    {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2, 3, 4]];
        let ret = matrix_reshape(mat, 1, 4);
        assert_eq!(ret, expected);
    }
    {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2], vec![3, 4]];
        let ret = matrix_reshape(mat, 2, 4);
        assert_eq!(ret, expected);
    }
}
