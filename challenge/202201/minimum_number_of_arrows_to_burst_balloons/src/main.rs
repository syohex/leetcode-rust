fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut count = 1;
    let mut end = points[0][1];
    for p in points.iter().skip(1) {
        if p[0] <= end {
            end = std::cmp::min(end, p[1]);
        } else {
            count += 1;
            end = p[1];
        }
    }

    count
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    println!("ret={}", find_min_arrow_shots(points));
}

#[test]
fn test_find_min_arrow_shots() {
    {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }
    {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(find_min_arrow_shots(points), 4);
    }
    {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }
}
