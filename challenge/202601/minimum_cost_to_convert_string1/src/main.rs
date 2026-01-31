fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    fn to_index(ch: char) -> usize {
        ch as usize - 'a' as usize
    }

    fn create_convertion_cost_table(from: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), from));

        let mut costs = vec![i64::MAX; 26];
        costs[from] = 0;

        while let Some((Reverse(cost), node)) = q.pop() {
            for (next_node, next_cost) in &graph[node] {
                let new_cost = cost + *next_cost;
                if new_cost < costs[*next_node] {
                    costs[*next_node] = new_cost;
                    q.push((Reverse(new_cost), *next_node));
                }
            }
        }

        costs
    }

    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; 26];
    for ((orig, chan), c) in original
        .into_iter()
        .zip(changed.into_iter())
        .zip(cost.into_iter())
    {
        let index = to_index(orig);
        let next = to_index(chan);
        graph[index].push((next, c as i64));
    }

    let mut ret = 0;
    let mut cost_tables: Vec<Option<Vec<i64>>> = vec![None; 26];
    for (from, to) in source.chars().zip(target.chars()) {
        let from = to_index(from);
        let to = to_index(to);
        let cost = if let Some(table) = &cost_tables[from] {
            table[to]
        } else {
            let table = create_convertion_cost_table(from, &graph);
            let v = table[to];
            cost_tables[from] = Some(table);
            v
        };

        if cost == i64::MAX {
            return -1;
        }

        ret += cost;
    }

    ret
}

fn main() {
    let source = "abcd".to_string();
    let target = "acbe".to_string();
    let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
    let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
    let cost = vec![2, 5, 5, 1, 2, 20];
    let ret = minimum_cost(source, target, original, changed, cost);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        let ret = minimum_cost(source, target, original, changed, cost);
        assert_eq!(ret, 28);
    }
    {
        let source = "aaaa".to_string();
        let target = "bbbb".to_string();
        let original = vec!['a', 'c'];
        let changed = vec!['c', 'b'];
        let cost = vec![1, 2];
        let ret = minimum_cost(source, target, original, changed, cost);
        assert_eq!(ret, 12);
    }
    {
        let source = "abcd".to_string();
        let target = "abce".to_string();
        let original = vec!['a'];
        let changed = vec!['e'];
        let cost = vec![10000];
        let ret = minimum_cost(source, target, original, changed, cost);
        assert_eq!(ret, -1);
    }
}
