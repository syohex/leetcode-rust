fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;

    let mut q = BinaryHeap::from_iter(gifts);
    for _i in 0..k {
        if let Some(v) = q.pop() {
            let v = (v as f64).sqrt() as i32;
            q.push(v);
        }
    }

    let mut ret = 0i64;
    while !q.is_empty() {
        ret += q.pop().unwrap() as i64;
    }
    ret
}

fn main() {
    let gifts = vec![25, 64, 9, 4, 100];
    let ret = pick_gifts(gifts, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let gifts = vec![25, 64, 9, 4, 100];
        let ret = pick_gifts(gifts, 4);
        assert_eq!(ret, 29);
    }
    {
        let gifts = vec![1, 1, 1, 1];
        let ret = pick_gifts(gifts, 4);
        assert_eq!(ret, 4);
    }
}
