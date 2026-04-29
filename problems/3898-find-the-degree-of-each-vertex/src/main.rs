fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    matrix.into_iter().map(|v| v.into_iter().sum()).collect()
}

fn main() {
    let matrix = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    let ret = find_degrees(matrix);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let matrix = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let ret = find_degrees(matrix);
        assert_eq!(ret, vec![2, 2, 2]);
    }
    {
        let matrix = vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]];
        let ret = find_degrees(matrix);
        assert_eq!(ret, vec![1, 1, 0]);
    }
    {
        let matrix = vec![vec![0]];
        let ret = find_degrees(matrix);
        assert_eq!(ret, vec![0]);
    }
}
