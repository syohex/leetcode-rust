fn find_cheapest_price(_n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for flight in flights {
        graph
            .entry(flight[0])
            .or_insert(vec![])
            .push((flight[1], flight[2]));
    }

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((src, 0));

    let mut ret = std::i32::MAX;
    let mut step = -1;
    while step <= k && !q.is_empty() {
        let len = q.len();

        for _ in 0..len {
            let (node, cost) = q.pop_front().unwrap();
            if node == dst {
                ret = std::cmp::min(ret, cost);
            }

            if let Some(nexts) = graph.get(&node) {
                for (next, price) in nexts {
                    q.push_back((*next, cost + *price));
                }
            }
        }

        step += 1;
    }

    if ret == std::i32::MAX {
        -1
    } else {
        ret
    }
}

fn main() {
    let n = 4;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src = 0;
    let dst = 3;
    let k = 1;
    let ret = find_cheapest_price(n, flights, src, dst, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        let ret = find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ret, 700);
    }
    {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let ret = find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ret, 200);
    }
    {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let ret = find_cheapest_price(n, flights, src, dst, k);
        assert_eq!(ret, 500);
    }
}
