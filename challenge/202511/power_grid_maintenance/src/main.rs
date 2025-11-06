fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[derive(Debug, Default, Clone)]
    struct Node {
        is_offline: bool,
        grid_id: usize,
    }

    fn traverse_grid(
        node: usize,
        id: usize,
        graph: &Vec<Vec<usize>>,
        nodes: &mut Vec<Node>,
        q: &mut BinaryHeap<Reverse<usize>>,
    ) {
        nodes[node].grid_id = id;
        q.push(Reverse(node));

        for &next in &graph[node] {
            if nodes[next].grid_id == 0 {
                traverse_grid(next, id, graph, nodes, q);
            }
        }
    }

    let c = c as usize;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; c + 1];
    for c in connections {
        graph[c[0] as usize].push(c[1] as usize);
        graph[c[1] as usize].push(c[0] as usize);
    }

    let mut grid_queue = vec![BinaryHeap::default(); c + 1];
    let mut nodes: Vec<Node> = vec![Node::default(); c + 1];
    let mut grid_id = 1;
    for i in 1..=c {
        if nodes[i].grid_id == 0 {
            let mut q = BinaryHeap::new();
            traverse_grid(i, grid_id, &graph, &mut nodes, &mut q);
            grid_queue[grid_id] = q;
            grid_id += 1;
        }
    }

    let mut ret = vec![];
    for query in queries {
        let (op, x) = (query[0], query[1] as usize);
        if op == 1 {
            if !nodes[x].is_offline {
                ret.push(x as i32);
            } else {
                let grid_id = nodes[x].grid_id;
                if let Some(q) = grid_queue.get_mut(grid_id) {
                    while let Some(Reverse(node)) = q.peek()
                        && nodes[*node].is_offline
                    {
                        q.pop();
                    }

                    if let Some(Reverse(node)) = q.peek() {
                        ret.push(*node as i32);
                    } else {
                        ret.push(-1);
                    }
                }
            }
        } else {
            nodes[x].is_offline = true;
        }
    }

    ret
}

fn main() {
    let c = 5;
    let connections = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    let queries = vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]];
    let ret = process_queries(c, connections, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let c = 2;
        let connections = vec![vec![1, 2]];
        let queries = vec![vec![1, 1], vec![1, 2], vec![2, 2], vec![2, 1], vec![1, 1]];
        let ret = process_queries(c, connections, queries);
        assert_eq!(ret, [1, 2, -1]);
    }
    {
        let c = 5;
        let connections = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let queries = vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]];
        let ret = process_queries(c, connections, queries);
        assert_eq!(ret, [3, 2, 3]);
    }
    {
        let c = 3;
        let connections = vec![];
        let queries = vec![vec![1, 1], vec![2, 1], vec![1, 1]];
        let ret = process_queries(c, connections, queries);
        assert_eq!(ret, [1, -1]);
    }
}
