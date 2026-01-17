fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let len = bottom_left.len();
    let mut max_len = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            let width = std::cmp::min(top_right[i][0], top_right[j][0])
                - std::cmp::max(bottom_left[i][0], bottom_left[j][0]);
            let height = std::cmp::min(top_right[i][1], top_right[j][1])
                - std::cmp::max(bottom_left[i][1], bottom_left[j][1]);
            let min_len = std::cmp::min(width, height);
            max_len = std::cmp::max(max_len, min_len);
        }
    }

    let max_len = max_len as i64;
    max_len * max_len
}

fn main() {
    let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
    let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
    let ret = largest_square_area(bottom_left, top_right);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
        let ret = largest_square_area(bottom_left, top_right);
        assert_eq!(ret, 1);
    }
    {
        let bottom_left = vec![vec![1, 1], vec![1, 3], vec![1, 5]];
        let top_right = vec![vec![5, 5], vec![5, 7], vec![5, 9]];
        let ret = largest_square_area(bottom_left, top_right);
        assert_eq!(ret, 4);
    }
    {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![1, 2]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![3, 4]];
        let ret = largest_square_area(bottom_left, top_right);
        assert_eq!(ret, 1);
    }
    {
        let bottom_left = vec![vec![1, 1], vec![3, 3], vec![3, 1]];
        let top_right = vec![vec![2, 2], vec![4, 4], vec![4, 2]];
        let ret = largest_square_area(bottom_left, top_right);
        assert_eq!(ret, 0);
    }
}
