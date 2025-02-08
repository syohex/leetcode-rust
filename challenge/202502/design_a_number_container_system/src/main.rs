use std::collections::{HashMap, HashSet};

struct NumberContainers {
    nums: HashMap<i32, i32>,
    index: HashMap<i32, HashSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            index: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        let old_number = if let Some(v) = self.nums.get(&index) {
            *v
        } else {
            0
        };

        self.nums.insert(index, number);
        if old_number != 0 {
            if let Some(v) = self.index.get_mut(&old_number) {
                v.remove(&index);
            }
        }

        self.index
            .entry(number)
            .or_default()
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(v) = self.index.get(&number) {
            if let Some(m) = v.iter().min() {
                *m
            } else {
                -1
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
