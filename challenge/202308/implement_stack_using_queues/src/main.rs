use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut tmp = VecDeque::new();
        while self.queue.len() > 1 {
            tmp.push_back(self.queue.pop_front().unwrap());
        }

        let ret = self.queue.pop_front().unwrap();
        while let Some(v) = tmp.pop_front() {
            self.queue.push_back(v);
        }

        ret
    }

    fn top(&self) -> i32 {
        *self.queue.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
fn main() {
    let mut m = MyStack::new();
    m.push(1);
    m.push(2);
    dbg!(m.top());
    dbg!(m.pop());
    dbg!(!m.empty());
    dbg!(m.top());
    dbg!(m.pop());
    dbg!(m.empty());
}

#[test]
fn test_my_stack() {
    {
        let mut m = MyStack::new();
        m.push(1);
        m.push(2);
        assert_eq!(m.top(), 2);
        assert_eq!(m.pop(), 2);
        assert!(!m.empty());
        assert_eq!(m.top(), 1);
        assert_eq!(m.pop(), 1);
        assert!(m.empty());
    }
}
