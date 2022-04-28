use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    h: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut h = BinaryHeap::new();
        let k = k as usize;

        for num in nums {
            h.push(Reverse(num));
        }

        while h.len() > k {
            h.pop();
        }

        Self { k: k, h: h }
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
    let ret = k.add(3);
    println!("ret={ret}");
}

#[test]
fn test_kth_largest() {
    {
        let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }
}
