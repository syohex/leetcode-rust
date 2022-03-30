fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut low = 0i32;
    let mut high = (rows - 1) as i32;

    while low < high {
        let mid = low + ((high - low) / 2);
        let mid_first = matrix[mid as usize][0];
        let mid_last = matrix[mid as usize][cols - 1];

        if mid_first == target {
            return true;
        }

        if target < mid_first {
            high = mid - 1;
        } else if target > mid_last {
            low = mid + 1;
        } else {
            low = mid;
            break;
        }
    }

    let low = low as usize;
    let mut left = 0i32;
    let mut right = (matrix[low].len() - 1) as i32;

    while left <= right {
        let mid = left + ((right - left) / 2);
        let mid_val = matrix[low][mid as usize];
        if mid_val == target {
            return true;
        }

        if target < mid_val {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    false
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let ret = search_matrix(matrix, 3);
    println!("ret={ret}");
}

#[test]
fn test_search_matrix() {
    {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(search_matrix(matrix.clone(), 3));
        assert!(search_matrix(matrix.clone(), 16));
        assert!(search_matrix(matrix.clone(), 30));
        assert!(search_matrix(matrix.clone(), 11));
        assert!(search_matrix(matrix.clone(), 20));
    }
    {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!search_matrix(matrix.clone(), 13));
        assert!(!search_matrix(matrix.clone(), 4));
        assert!(!search_matrix(matrix.clone(), 15));
        assert!(!search_matrix(matrix.clone(), 22));
        assert!(!search_matrix(matrix.clone(), 0));
        assert!(!search_matrix(matrix.clone(), 9));
    }
}
