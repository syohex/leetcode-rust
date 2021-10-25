use std::cmp::min;

struct MinStack {
    vec: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if self.vec.is_empty() {
            self.vec.push((val, val));
        } else {
            let min_val = min(self.get_min(), val);
            self.vec.push((val, min_val));
        }
    }

    fn pop(&mut self) {
        self.vec.pop().unwrap();
    }

    fn top(&self) -> i32 {
        let last = self.vec.len() - 1;
        self.vec[last].0
    }

    fn get_min(&self) -> i32 {
        let last = self.vec.len() - 1;
        self.vec[last].1
    }
}

fn main() {
    let mut m = MinStack::new();
    m.push(1);
    println!("ret={}", m.get_min());
    println!("ret={}", m.top());
    m.pop();
}

#[test]
fn test_min_stack() {
    {
        let mut m = MinStack::new();
        m.push(-2);
        m.push(0);
        m.push(-3);
        assert_eq!(m.get_min(), -3);
        m.pop();
        assert_eq!(m.top(), 0);
        assert_eq!(m.get_min(), -2);
    }
}
