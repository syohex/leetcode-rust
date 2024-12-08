fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut events = events;
    events.sort_unstable();

    let mut q = BinaryHeap::new();
    let mut ret = 0;
    let mut max_value = 0;
    for e in events {
        let (start, end, value) = (e[0], e[1], e[2]);
        while let Some(Reverse((end2, value2))) = q.peek() {
            if start <= *end2  {
                break;
            }

            max_value = std::cmp::max(max_value, *value2);
            q.pop();
        }

        ret = std::cmp::max(ret, max_value + value);
        q.push(Reverse((end, value)));
    }

    ret
}

fn main() {
    let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]];
    let ret = max_two_events(events);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]];
        let ret = max_two_events(events);
        assert_eq!(ret, 4);
    }
    {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]];
        let ret = max_two_events(events);
        assert_eq!(ret, 5);
    }
    {
        let events = vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]];
        let ret = max_two_events(events);
        assert_eq!(ret, 8);
    }
}
