fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut left = 0 as i32;
    let mut right = (rows * cols - 1) as i32;

    while left <= right {
        let mid = left + (right - left) / 2;
        let v = matrix[mid as usize / cols][mid as usize % cols];
        if v == target {
            return true;
        }

        if target < v {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    false
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let ret = search_matrix(matrix, 16);
    println!("ret={ret}");
}

#[test]
fn test_search_matrix() {
    {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(search_matrix(matrix, 3));
    }
    {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!search_matrix(matrix, 13));
    }
    {
        let matrix = vec![vec![1]];
        assert!(!search_matrix(matrix, 0));
    }
}
