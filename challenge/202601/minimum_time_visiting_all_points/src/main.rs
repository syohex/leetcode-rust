fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    for i in 1..points.len() {
        ret += std::cmp::max(
            (points[i][0] - points[i - 1][0]).abs(),
            (points[i][1] - points[i - 1][1]).abs(),
        );
    }

    ret
}

fn main() {
    let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
    let ret = min_time_to_visit_all_points(points);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        let ret = min_time_to_visit_all_points(points);
        assert_eq!(ret, 7);
    }
    {
        let points = vec![vec![3, 2], vec![-2, 2]];
        let ret = min_time_to_visit_all_points(points);
        assert_eq!(ret, 5);
    }
}
