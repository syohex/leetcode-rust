fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for row in matrix.iter().rev() {
        if target >= row[0] {
            if row.iter().find(|&e| *e == target).is_some() {
                return true;
            }
        }
    }

    false
}

fn main() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    let ret = search_matrix(matrix, 5);
    println!("ret={}", ret);
}

#[test]
fn test_search_matrix() {
    {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(search_matrix(matrix, 5));
    }
    {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(!search_matrix(matrix, 20));
    }
}
