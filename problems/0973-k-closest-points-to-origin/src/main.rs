use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let v1 = ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt();
        let v2 = ((other.x.pow(2) + other.y.pow(2)) as f64).sqrt();
        if v1 == v2 {
            Ordering::Equal
        } else if v1 > v2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut priority_queue = BinaryHeap::new();
    for point in &points {
        priority_queue.push(Coordinate {
            x: point[0],
            y: point[1],
        })
    }

    let mut ret = vec![];
    for _ in 0..k {
        let c = priority_queue.pop().unwrap();
        ret.push(vec![c.x, c.y]);
    }
    ret
}

fn main() {
    let input = vec![vec![1, 3], vec![-2, 2]];
    let ret = k_closest(input, 1);
    println!("ret={:?}", ret);
}

#[test]
fn test_k_closest() {
    {
        let input = vec![vec![1, 3], vec![-2, 2]];
        let ret = k_closest(input, 1);
        assert_eq!(ret.len(), 1);
        assert_eq!(ret[0], vec![-2, 2]);
    }
    {
        let input = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let ret = k_closest(input, 2);
        assert_eq!(ret.len(), 2);
        assert!(ret.iter().find(|&x| *x == vec![3, 3]).is_some());
        assert!(ret.iter().find(|&x| *x == vec![-2, 4]).is_some());
    }
}
