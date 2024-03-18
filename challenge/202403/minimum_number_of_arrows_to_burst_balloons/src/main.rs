fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_unstable_by_key(|v| v[1]);

    let mut ret = 1;
    let mut end = points[0][1];

    for point in points {
        if point[0] > end {
            ret += 1;
            end = point[1];
        }
    }

    ret
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let ret = find_min_arrow_shots(points);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let ret = find_min_arrow_shots(points);
        assert_eq!(ret, 2);
    }
    {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let ret = find_min_arrow_shots(points);
        assert_eq!(ret, 4);
    }
    {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let ret = find_min_arrow_shots(points);
        assert_eq!(ret, 2);
    }
}
