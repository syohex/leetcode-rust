use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    h: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut h = BinaryHeap::new();
        for num in nums {
            h.push(Reverse(num));
        }

        let k = k as usize;
        while h.len() > k {
            h.pop();
        }

        Self { h, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.h.push(Reverse(val));

        while self.h.len() > self.k {
            self.h.pop();
        }

        self.h.peek().unwrap().0
    }
}

fn main() {
    let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("ret={}", k.add(3));
}

#[test]
fn test_kth_larges() {
    {
        let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }
}
