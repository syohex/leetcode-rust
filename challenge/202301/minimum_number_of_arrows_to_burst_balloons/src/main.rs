fn find_min_arrows_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_unstable_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut ret = 1;
    let mut start = points[0][0];
    let mut end = points[0][1];
    for v in points.into_iter().skip(1) {
        let s = std::cmp::max(start, v[0]);
        let e = std::cmp::min(end, v[1]);

        if s > e {
            ret += 1;
            start = v[0];
            end = v[1];
        } else {
            start = s;
            end = e;
        }
    }

    ret
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let ret = find_min_arrows_shots(points);
    println!("ret={ret}");
}

#[test]
fn test_find_min_arrow_shots() {
    {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let ret = find_min_arrows_shots(points);
        assert_eq!(ret, 2);
    }
    {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let ret = find_min_arrows_shots(points);
        assert_eq!(ret, 4);
    }
    {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let ret = find_min_arrows_shots(points);
        assert_eq!(ret, 2);
    }
}
