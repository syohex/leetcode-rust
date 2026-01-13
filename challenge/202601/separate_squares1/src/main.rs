fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut total_area = 0.0;
    let mut max_y = i32::MIN;

    for s in &squares {
        let len= s[2] as f64;
        total_area += len * len;
        max_y = std::cmp::max(max_y, s[1] + s[2]);
    }

    let mut left = 0f64;
    let mut right = max_y as f64;
    let epsilon = 0.00001;
    let half_area = total_area / 2.0;

    while (right - left).abs() > epsilon {
        let mid = left + (right - left) / 2.0;

        let mut area = 0.0;
        for s in &squares {
            let (y, len) = (s[1] as f64, s[2] as f64);
            if y < mid {
                if y + len > mid {
                    area += len * (mid - y);
                } else {
                    area += len * len;
                }
            }
        }

        if area >= half_area {
            right = mid;
        } else {
            left = mid;
        }
    }

    right
}

fn main() {
    let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
    let ret = separate_squares(squares);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let squares = vec![vec![0, 0, 2], vec![1, 1, 1]];
        let ret = separate_squares(squares);
        assert!((ret - (7.0 / 6.0)).abs() <= 0.00001);
    }
    {
        let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
        let ret = separate_squares(squares);
        assert!((ret - 1.0).abs() <= 0.00001)
    }
}
