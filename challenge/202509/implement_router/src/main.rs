use std::collections::{HashMap, HashSet, VecDeque};
struct Router {
    limit: usize,
    packets: HashSet<(i32, i32, i32)>,
    queue: VecDeque<(i32, i32, i32)>,
    dest_queue: HashMap<i32, VecDeque<i32>>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            limit: memory_limit as usize,
            packets: HashSet::new(),
            queue: VecDeque::new(),
            dest_queue: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if self.packets.contains(&(source, destination, timestamp)) {
            return false;
        }

        if self.queue.len() >= self.limit
            && let Some((src, dest, time)) = self.queue.pop_front()
            && let Some(q) = self.dest_queue.get_mut(&dest)
        {
            q.pop_front();
            self.packets.remove(&(src, dest, time));
        }

        self.packets.insert((source, destination, timestamp));
        self.queue.push_back((source, destination, timestamp));
        self.dest_queue
            .entry(destination)
            .or_default()
            .push_back(timestamp);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((src, dest, time)) = self.queue.pop_front() {
            if let Some(q) = self.dest_queue.get_mut(&dest) {
                q.pop_front();
            }
            self.packets.remove(&(src, dest, time));

            vec![src, dest, time]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(q) = self.dest_queue.get(&destination) {
            let lower = q.partition_point(|&n| n < start_time);
            let upper = q.partition_point(|&n| n <= end_time);

            (upper - lower) as i32
        } else {
            0
        }
    }
}

fn main() {
    let mut r = Router::new(3);
    r.add_packet(1, 4, 90);
    r.add_packet(2, 5, 90);
    r.add_packet(1, 4, 90);
    r.add_packet(3, 5, 95);
    r.add_packet(4, 5, 105);
    let v = r.forward_packet();
    assert_eq!(v, vec![2, 5, 90]);
    r.add_packet(5, 2, 110);
    assert_eq!(r.get_count(5, 100, 110), 1);
}

#[test]
fn test() {
    {
        let mut r = Router::new(3);
        assert!(r.add_packet(1, 4, 90));
        assert!(r.add_packet(2, 5, 90));
        assert!(r.add_packet(3, 5, 90));
        assert!(r.add_packet(4, 5, 90));
        assert!(r.add_packet(1, 4, 90));
    }
    {
        let mut r = Router::new(2);
        r.add_packet(4, 2, 1);
        assert_eq!(r.get_count(2, 1, 1), 1);
        let v = r.forward_packet();
        assert_eq!(v, vec![4, 2, 1]);
        assert_eq!(r.get_count(2, 1, 1), 0);
        r.add_packet(4, 2, 1);
        assert_eq!(r.get_count(2, 1, 1), 1);
    }
    {
        let mut r = Router::new(2);
        r.add_packet(3, 1, 3);
        r.add_packet(1, 2, 3);
        r.add_packet(4, 5, 3);
        dbg!(&r.dest_queue);
        assert_eq!(r.get_count(1, 2, 3), 0);
    }
    {
        let mut r = Router::new(3);
        r.add_packet(1, 4, 90);
        r.add_packet(2, 5, 90);
        r.add_packet(1, 4, 90);
        r.add_packet(3, 5, 95);
        r.add_packet(4, 5, 105);
        let v = r.forward_packet();
        assert_eq!(v, vec![2, 5, 90]);
        r.add_packet(5, 2, 110);
        assert_eq!(r.get_count(5, 100, 110), 1);
    }
}
