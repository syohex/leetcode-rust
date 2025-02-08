use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct NumberContainers {
    nums: HashMap<i32, i32>,
    index: HashMap<i32, BinaryHeap<Reverse<i32>>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            index: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        self.nums.insert(index, number);
        self.index.entry(number).or_default().push(Reverse(index));
    }

    fn find(&mut self, number: i32) -> i32 {
        if let Some(q) = self.index.get_mut(&number) {
            loop {
                if let Some(Reverse(n)) = q.peek() {
                    let v = *self.nums.get(n).unwrap();
                    if v == number {
                        return *n;
                    }

                    q.pop();
                } else {
                    return -1;
                }
            }
        } else {
            -1
        }
    }
}

fn main() {
    let mut nc = NumberContainers::new();
    nc.change(2, 10);
    nc.change(1, 10);
    nc.change(3, 10);
    nc.change(5, 10);
    nc.change(1, 20);
    let ret = nc.find(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let mut nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1);
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(nc.find(10), 1);
        nc.change(1, 20);
        assert_eq!(nc.find(10), 2);
    }
}
