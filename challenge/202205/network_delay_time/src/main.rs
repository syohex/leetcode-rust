fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[derive(Eq, PartialEq)]
    struct Data {
        node: i32,
        cost: i32,
    }

    impl Ord for Data {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for Data {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for time in times.iter() {
        if let Some(v) = graph.get_mut(&time[0]) {
            v.push((time[1], time[2]));
        } else {
            graph.insert(time[0], vec![(time[1], time[2])]);
        }
    }

    let mut q = BinaryHeap::new();
    q.push(Data { node: k, cost: 0 });

    let mut visited = HashSet::new();
    let mut ret = -1;
    while let Some(d) = q.pop() {
        if visited.contains(&d.node) {
            continue;
        }

        visited.insert(d.node);
        ret = std::cmp::max(ret, d.cost);

        if let Some(v) = graph.get(&d.node) {
            for (next, cost) in v {
                q.push(Data {
                    node: *next,
                    cost: d.cost + cost,
                });
            }
        }
    }

    if visited.len() == n as usize {
        ret
    } else {
        -1
    }
}

fn main() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let ret = network_delay_time(times, 4, 2);
    println!("ret={ret}");
}

#[test]
fn test_network_delay_time() {
    {
        let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        assert_eq!(network_delay_time(times, 4, 2), 2);
    }
    {
        let times = vec![vec![1, 2, 1]];
        assert_eq!(network_delay_time(times, 2, 1), 1);
    }
    {
        let times = vec![vec![1, 2, 1]];
        assert_eq!(network_delay_time(times, 2, 2), -1);
    }
    {
        let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]];
        assert_eq!(network_delay_time(times, 3, 1), 3);
    }
}
