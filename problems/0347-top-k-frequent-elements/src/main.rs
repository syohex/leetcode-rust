use std::collections::{BinaryHeap, HashMap};

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut h: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        *h.entry(n).or_insert(0) += 1;
    }

    let mut v: Vec<Vec<&i32>> = vec![];
    for (n, count) in h.iter() {
        v.push(vec![n, count]);
    }

    v.sort_by(|a, b| b[1].partial_cmp(a[1]).unwrap());
    println!("v={:?}", v);

    let mut heap = BinaryHeap::new();
    for e in v {
        heap.push(vec![e[1], e[0]]);
    }

    let mut ret: Vec<i32> = vec![];
    for _ in 0..k {
        ret.push(*heap.pop().unwrap()[1]);
    }

    ret
}

fn main() {
    println!(
        "top_k_frequent([1, 1, 1, 2, 2, 3], 2)={:?}",
        top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
    );
}

#[test]
fn test_k_frequent() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
}
