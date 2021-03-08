fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let mut min = std::i32::MAX;
    let mut ret = -1;
    for (i, p) in points.iter().enumerate() {
        if !(p[0] == x || p[1] == y) {
            continue;
        }

        let dist = (p[0] - x).abs() + (p[1] - y).abs();
        if dist < min {
            min = dist;
            ret = i as i32;
        }
    }

    ret
}

fn main() {
    let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
    let ret = nearest_valid_point(3, 4, points);
    println!("ret={}", ret);
}

#[test]
fn test_nearest_valid_point() {
    {
        let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        assert_eq!(nearest_valid_point(3, 4, points), 2);
    }
    {
        let points = vec![vec![3, 4]];
        assert_eq!(nearest_valid_point(3, 4, points), 0);
    }
    {
        let points = vec![vec![2, 3]];
        assert_eq!(nearest_valid_point(3, 4, points), -1);
    }
}
