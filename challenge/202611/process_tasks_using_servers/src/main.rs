fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut availables = BinaryHeap::new();

    for (i, weight) in servers.iter().enumerate() {
        availables.push(Reverse((*weight, i)));
    }

    let mut unavailables = BinaryHeap::new();
    let mut ret = vec![];
    let mut time = 0;

    for (i, task) in tasks.into_iter().enumerate() {
        time = std::cmp::max(time, i as i32);
        while let Some(Reverse((t, _))) = unavailables.peek()
            && *t <= time
            && let Some(Reverse((_, id))) = unavailables.pop()
        {
            availables.push(Reverse((servers[id], id)));
        }

        if availables.is_empty() {
            if let Some(Reverse((t, _))) = unavailables.peek() {
                time = *t;
            }

            while let Some(Reverse((t, _))) = unavailables.peek()
                && *t <= time
                && let Some(Reverse((_, id))) = unavailables.pop()
            {
                availables.push(Reverse((servers[id], id)));
            }
        }

        if let Some(Reverse((_, id))) = availables.pop() {
            ret.push(id as i32);
            unavailables.push(Reverse((time + task, id)));
        }
    }

    ret
}

fn main() {
    let servers = vec![3, 3, 2];
    let tasks = vec![1, 2, 3, 2, 1, 2];
    let ret = assign_tasks(servers, tasks);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let servers = vec![3, 3, 2];
        let tasks = vec![1, 2, 3, 2, 1, 2];
        let expected = vec![2, 2, 0, 2, 1, 2];
        let ret = assign_tasks(servers, tasks);
        assert_eq!(ret, expected);
    }
    {
        let servers = vec![5, 1, 4, 3, 2];
        let tasks = vec![2, 1, 2, 4, 5, 2, 1];
        let expected = vec![1, 4, 1, 4, 1, 3, 2];
        let ret = assign_tasks(servers, tasks);
        assert_eq!(ret, expected);
    }
}
