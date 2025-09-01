fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    use std::collections::BinaryHeap;

    fn diff(pass: i32, total: i32) -> i64 {
        let pass = pass as i64;
        let total = total as i64;
        let v = 1_000_000_000;

        v * (pass + 1) / (total + 1) - v * pass / total
    }

    let mut b = BinaryHeap::new();
    for c in &classes {
        b.push((diff(c[0], c[1]), c[0], c[1]));
    }

    for _ in 0..extra_students {
        if let Some((_, pass, total)) = b.pop() {
            b.push((diff(pass + 1, total + 1), pass + 1, total + 1));
        }
    }

    let mut ret = 0.0;
    while let Some((_, pass, total)) = b.pop() {
        ret += pass as f64 / total as f64;
    }

    ret / classes.len() as f64
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
        assert!((ret - 0.78333).abs() <= 0.00001);
    }
    {
        let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
        let ret = max_average_ratio(classes, 4);
        assert!((ret - 0.53485).abs() <= 0.00001);
    }
}
