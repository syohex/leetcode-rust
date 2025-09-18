use std::collections::{BinaryHeap, HashMap};

struct TaskManager {
    task_priority: HashMap<i32, i32>,
    task_user: HashMap<i32, i32>,
    priority_queue: BinaryHeap<(i32, i32, i32)>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_priority = HashMap::new();
        let mut task_user = HashMap::new();
        let mut priority_queue = BinaryHeap::new();

        for task in tasks {
            task_priority.insert(task[1], task[2]);
            task_user.insert(task[1], task[0]);
            priority_queue.push((task[2], task[1], task[0]));
        }

        Self {
            task_priority,
            task_user,
            priority_queue,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_user.insert(task_id, user_id);
        self.task_priority.insert(task_id, priority);
        self.priority_queue.push((priority, task_id, user_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(user_id) = self.task_user.get(&task_id) {
            self.task_priority.insert(task_id, new_priority);
            self.priority_queue.push((new_priority, task_id, *user_id));
        }
    }

    fn rmv(&mut self, task_id: i32) {
        self.task_priority.insert(task_id, -1);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id, user_id)) = self.priority_queue.pop() {
            if let Some(current_priority) = self.task_priority.get(&task_id)
                && *current_priority == priority
            {
                self.task_priority.insert(task_id, -1);
                return user_id;
            }
        }

        -1
    }
}

fn main() {
    let data = vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]];

    let mut t = TaskManager::new(data);
    t.add(4, 104, 5);
    t.edit(102, 8);
    assert_eq!(t.exec_top(), 3);
    t.rmv(101);
    t.add(5, 105, 15);
    assert_eq!(t.exec_top(), 5);
}

#[test]
fn test() {
    {
        let data = vec![vec![4, 14, 27]];

        let mut t = TaskManager::new(data);
        t.add(7, 4, 15);
        t.rmv(4);
        t.add(0, 15, 33);
        t.edit(15, 16);
        assert_eq!(t.exec_top(), 4);
        t.add(1, 7, 36);
        t.edit(15, 37);
        assert_eq!(t.exec_top(), 0);
    }
    {
        let data = vec![vec![1, 101, 8], vec![2, 102, 20], vec![3, 103, 5]];

        let mut t = TaskManager::new(data);
        t.add(4, 104, 5);
        t.edit(102, 9);
        assert_eq!(t.exec_top(), 2);
        t.rmv(101);
        t.add(50, 101, 8);
        assert_eq!(t.exec_top(), 50);
    }
    {
        let data = vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]];

        let mut t = TaskManager::new(data);
        t.add(4, 104, 5);
        t.edit(102, 8);
        assert_eq!(t.exec_top(), 3);
        t.rmv(101);
        t.add(5, 105, 15);
        assert_eq!(t.exec_top(), 5);
    }
}
