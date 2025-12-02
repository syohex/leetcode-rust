fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut ys = HashMap::new();
    for p in points {
        *ys.entry(p[1]).or_insert(0i64) += 1;
    }

    let modulo = 1_000_000_007i64;
    let mut total_edges = 0i64;
    let mut ret = 0i64;

    for y in ys.into_values() {
        let edges = (y * (y - 1)) / 2;
        ret += edges * total_edges;
        total_edges += edges;
    }

    (ret % modulo) as i32
}

fn main() {
    let points = vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]];
    let ret = count_trapezoids(points);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let points = vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]];
        let ret = count_trapezoids(points);
        assert_eq!(ret, 3);
    }
    {
        let points = vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]];
        let ret = count_trapezoids(points);
        assert_eq!(ret, 1);
    }
}
