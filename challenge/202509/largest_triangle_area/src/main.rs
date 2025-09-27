fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    fn area(x: &[i32], y: &[i32], z: &[i32]) -> f64 {
        0.5 * (x[0] * y[1] - y[0] * x[1] + y[0] * z[1] - z[0] * y[1] + z[0] * x[1] - x[0] * z[1])
            .abs() as f64
    }

    let len = points.len();
    let mut ret = 0.0;
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            for k in (j + 1)..len {
                let a = area(&points[i], &points[j], &points[k]);
                if a > ret {
                    ret = a;
                }
            }
        }
    }

    ret
}

fn main() {
    let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
    let ret = largest_triangle_area(points);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        let ret = largest_triangle_area(points);
        assert_eq!(ret, 2.0);
    }
    {
        let points = vec![vec![1, 0], vec![0, 0], vec![0, 1]];
        let ret = largest_triangle_area(points);
        assert_eq!(ret, 0.5);
    }
}
