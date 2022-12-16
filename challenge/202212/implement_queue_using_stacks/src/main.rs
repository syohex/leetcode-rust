struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        while self.stack1.len() > 1 {
            let v = self.stack1.pop().unwrap();
            self.stack2.push(v);
        }

        let ret = self.stack1.pop().unwrap();
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }

        ret
    }

    fn peek(&mut self) -> i32 {
        while self.stack1.len() > 1 {
            let v = self.stack1.pop().unwrap();
            self.stack2.push(v);
        }

        let ret = *self.stack1.last().unwrap();
        while !self.stack2.is_empty() {
            self.stack1.push(self.stack2.pop().unwrap());
        }

        ret
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}
fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    let peek = q.peek();
    let pop = q.pop();
    let empty = q.empty();
    println!("peek={peek}, pop={pop}, empty={empty}");
}

#[test]
fn test_queue() {
    {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
