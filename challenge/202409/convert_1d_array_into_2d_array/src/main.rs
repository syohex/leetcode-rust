fn construct2d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let (m, n) = (m as usize, n as usize);
    if original.len() != m * n {
        return vec![];
    }

    let mut ret = vec![vec![0; n]; m];
    for (i, v) in original.into_iter().enumerate() {
        ret[i / n][i % n] = v;
    }

    ret
}

fn main() {
    let original = vec![1, 2, 3, 4];
    let ret = construct2d_array(original, 2, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let original = vec![1, 2, 3, 4];
        let expected = vec![vec![1, 2], vec![3, 4]];
        let ret = construct2d_array(original, 2, 2);
        assert_eq!(ret, expected);
    }
    {
        let original = vec![1, 2, 3];
        let expected = vec![vec![1, 2, 3]];
        let ret = construct2d_array(original, 1, 3);
        assert_eq!(ret, expected);
    }
    {
        let original = vec![1, 2];
        let ret = construct2d_array(original, 1, 1);
        assert!(ret.is_empty());
    }
}
