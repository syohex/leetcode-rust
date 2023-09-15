fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let len = points.len();
    let mut checked = vec![false; len];
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0)));

    let mut ret = 0;
    while let Some(Reverse((weight, node))) = q.pop() {
        if checked[node] {
            continue;
        }

        checked[node] = true;
        ret += weight;

        for i in 0..len {
            if !checked[i] {
                let w =
                    (points[node][0] - points[i][0]).abs() + (points[node][1] - points[i][1]).abs();
                q.push(Reverse((w, i)));
            }
        }
    }

    ret
}

fn main() {
    let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    let ret = min_cost_connect_points(points);
    println!("ret={ret}");
}

#[test]
fn test_min_cost_connect_points() {
    {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        let ret = min_cost_connect_points(points);
        assert_eq!(ret, 20);
    }
    {
        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
        let ret = min_cost_connect_points(points);
        assert_eq!(ret, 18);
    }
}
