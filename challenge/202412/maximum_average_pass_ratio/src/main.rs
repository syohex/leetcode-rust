fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    use std::collections::BinaryHeap;

    fn diff(n: i32, d: i32) -> i64 {
        let k = 100_000_000i64;
        let next = (n + 1) as i64 * k / (d + 1) as i64;
        let current = n as i64 * k / d as i64;
        next - current
    }

    let len = classes.len();
    let mut q = BinaryHeap::new();
    for v in classes {
        q.push((diff(v[0], v[1]), v[0], v[1]))
    }

    for _ in 0..extra_students {
        if let Some((_, n, d)) = q.pop() {
            q.push((diff(n + 1, d + 1), n + 1, d + 1));
        }
    }

    let mut ret = 0f64;
    while let Some((_, n, d)) = q.pop() {
        ret += n as f64 / d as f64;
    }

    ret / len as f64
}
fn main() {
    let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
    let ret = max_average_ratio(classes, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let ret = max_average_ratio(classes, 2);
        let s = format!("{:.5}", ret);
        assert_eq!(s, "0.78333");
    }
    {
        let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
        let ret = max_average_ratio(classes, 4);
        let s = format!("{:.5}", ret);
        assert_eq!(s, "0.53485");
    }
    {
        let classes = vec![
            vec![280, 872],
            vec![108, 128],
            vec![3, 665],
            vec![93, 972],
            vec![347, 464],
            vec![443, 584],
            vec![809, 999],
            vec![366, 398],
        ];
        let ret = max_average_ratio(classes, 77862);
        let s = format!("{:.5}", ret);
        assert_eq!(s, "0.97596");
    }
}
